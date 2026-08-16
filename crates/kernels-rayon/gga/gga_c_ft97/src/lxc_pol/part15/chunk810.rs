//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 810/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk810(t10845: f64, t1268: f64, t4965: f64, t10850: f64, t21181: f64, t2917: f64, t10838: f64, t18877: f64, t18900: f64, t18902: f64, t21839: f64, t21843: f64, t21847: f64, t21852: f64, t21856: f64, t2265: f64, t631: f64) -> (f64, f64, f64) {
    let t21863 = t10845 * t4965 * t1268;
    let t21867 = t2917 * t10850 * t21181;
    let t21870 = t631 * t21839 / 2.0_f64 - 9.0_f64 / 2.0_f64 * t631 * t21843 + t631 * t21847 / 6.0_f64 + 6.0_f64 * t631 * t21852 + 2.0_f64 / 27.0_f64 * t631 * t21856 + t10838 + 4.0_f64 / 3.0_f64 * t18877 + 2.0_f64 / 3.0_f64 * t18900 - t18902 / 3.0_f64 - t2265 * t21863 / 3.0_f64 - t631 * t21867 / 3.0_f64;
    (t21863, t21867, t21870)
}
