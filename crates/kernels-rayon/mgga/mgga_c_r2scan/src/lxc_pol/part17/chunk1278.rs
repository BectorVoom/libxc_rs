//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1278/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1278(t37477: f64, t37483: f64, t39081: f64, t39083: f64, t40451: f64, t40485: f64, t42215: f64, t42216: f64, t43887: f64, t43892: f64, t43895: f64, t43898: f64, t44882: f64, t44885: f64, t44888: f64) -> f64 {
    let t45006 = -t39081 - 0.70441376091769752081e-2_f64 * t37477 + t44882 - 0.60975299583150056624e-3_f64 * t40451 - t42215 + t42216 + t39083 + t44885 + t44888 - 0.39032073591371545778e-3_f64 * t37483 + 0.59620292925746722033e-2_f64 * t40485 - 0.72042316457491791901e-3_f64 * t43887 - 0.72042316457491791901e-3_f64 * t43892 - 0.1440846329149835838e-2_f64 * t43895 - 0.1440846329149835838e-2_f64 * t43898;
    t45006
}
