//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta869 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2766;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta869<F: Float>(t1412: F, t6861: F, t2661: F, t3938: F, t3992: F, t5608: F, t5659: F, t1399: F, t22025: F, t22212: F, t2496: F, t1317: F, t22193: F, t2626: F, t1320: F, t22195: F, t221: F, t22253: F, t4018: F, t4019: F, t125: F, t21969: F, t6883: F, t9816: F, t9818: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t74026, t74029, t74033, t74037, t74106, t74111) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2766::<F>(t1412, t6861, t2661, t3938, t3992, t5608, t5659, t1399, t22025, t22212, t2496, t1317, t22193);
        let (t74130, t74132, t74174, t74177, t74184) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2767::<F>(t22212, t2626, t1320, t22195, t221, t22253, t4018, t4019, t125, t21969, t1399, t6883, t9816, t9818);
    (t74026, t74029, t74033, t74037, t74106, t74111, t74130, t74132, t74174, t74177, t74184)
}
