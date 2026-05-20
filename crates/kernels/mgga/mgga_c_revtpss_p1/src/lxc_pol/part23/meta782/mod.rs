//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta782 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2590;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta782<F: Float>(t45832: F, t460: F, t487: F, t5219: F, t5462: F, t1209: F, t21451: F, t17191: F, t3566: F, t3781: F, t5216: F, t45618: F, t43350: F, t44535: F, t45607: F, t13045: F, t13147: F, t1770: F, t1284: F, t5412: F, t17306: F, t3754: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t59737, t59749, t59788, t59817, t59854, t59864) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2590::<F>(t45832, t460, t487, t5219, t5462, t1209, t21451, t17191, t3566, t3781, t5216, t45618);
        let (t59865, t59871, t59872, t59948, t60008, t60019) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2591::<F>(t43350, t44535, t45607, t460, t487, t13045, t13147, t1770, t1209, t1284, t5412, t17306, t3754);
    (t59737, t59749, t59788, t59817, t59854, t59864, t59865, t59871, t59872, t59948, t60008, t60019)
}
