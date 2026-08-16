//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1297/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1297(t20019: f64, t33294: f64, t7292: f64, t11061: f64, t14555: f64, t32214: f64, t739: f64, t1890: f64, t3487: f64, t5241: f64, t23000: f64, t7805: f64) -> (f64, f64, f64, f64, f64) {
    let t33297 = 0.95334639871601137784e0_f64 * t33294 * t20019 * t7292;
    let t33299 = 0.15337170381568299871e1_f64 * t14555 * t11061;
    let t33300 = t739 * t32214;
    let t33304 = t1890 * t32214;
    let t33308 = t5241 * t3487;
    let t33310 = t23000 * t33308 * t7805;
    (t33297, t33299, t33300, t33304, t33310)
}
