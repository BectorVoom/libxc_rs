//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta852 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2737;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2738;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta852(t127: f64, t17693: f64, t20944: f64, t20946: f64, t1285: f64, t57659: f64, t17350: f64, t17934: f64, t17445: f64, t5373: f64, t12866: f64, t20933: f64, t56756: f64, t17789: f64, t21017: f64, t12916: f64, t17747: f64, t20962: f64, t3717: f64, t70994: f64, t1261: f64, t20867: f64, t3172: f64, t12956: f64, t20783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71435, t71440, t71447, t71460, t71470) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2737(t127, t17693, t20944, t20946, t1285, t57659, t17350, t17934, t17445, t5373, t12866, t20933, t56756);
        let (t71476, t71490, t71513, t71539, t71541) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2738(t17789, t21017, t12916, t17747, t20962, t3717, t70994, t1261, t20867, t3172, t12956, t20783);
    (t71435, t71440, t71447, t71460, t71470, t71476, t71490, t71513, t71539, t71541)
}
