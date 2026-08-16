//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1286/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1286(t321: f64, t3431: f64, t107: f64, t787: f64, t2028: f64, t3038: f64, t7275: f64, t1445: f64, t2034: f64, t2087: f64, t28381: f64, t32191: f64, t33105: f64, t33109: f64, t33112: f64, t33114: f64, t33117: f64, t33118: f64, t33126: f64, t33127: f64, t33130: f64, t33132: f64, t33134: f64, t33136: f64, t723: f64, t833: f64) -> (f64, f64) {
    let t33137 = t321 * t3431;
    let t33139 = t787 * t33137 * t107;
    let t33145 = 0.79445533226334281486e-1_f64 * t787 * t7275 * t3038 * t2028;
    let t33146 = -t33105 + t33109 - t33112 - t33114 + t33117 - 0.13803453343411469884e2_f64 * t2087 * t1445 * t33118 * t723 + 0.23005755572352449806e2_f64 * t833 * t1445 * t32191 - t33126 - t33127 + t28381 - t33130 - t33132 + t33134 + t33136 + 0.23833659967900284446e0_f64 * t33139 * t2034 - t33145;
    (t33137, t33146)
}
