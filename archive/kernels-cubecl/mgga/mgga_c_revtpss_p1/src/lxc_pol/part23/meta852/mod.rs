//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta852 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2737;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2738;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta852<F: Float>(t127: F, t17693: F, t20944: F, t20946: F, t1285: F, t57659: F, t17350: F, t17934: F, t17445: F, t5373: F, t12866: F, t20933: F, t56756: F, t17789: F, t21017: F, t12916: F, t17747: F, t20962: F, t3717: F, t70994: F, t1261: F, t20867: F, t3172: F, t12956: F, t20783: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t71435, t71440, t71447, t71460, t71470) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2737::<F>(t127, t17693, t20944, t20946, t1285, t57659, t17350, t17934, t17445, t5373, t12866, t20933, t56756);
        let (t71476, t71490, t71513, t71539, t71541) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2738::<F>(t17789, t21017, t12916, t17747, t20962, t3717, t70994, t1261, t20867, t3172, t12956, t20783);
    (t71435, t71440, t71447, t71460, t71470, t71476, t71490, t71513, t71539, t71541)
}
