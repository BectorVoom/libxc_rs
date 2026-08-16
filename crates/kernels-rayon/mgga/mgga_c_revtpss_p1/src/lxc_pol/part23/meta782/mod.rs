//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta782 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2590;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta782(t45832: f64, t460: f64, t487: f64, t5219: f64, t5462: f64, t1209: f64, t21451: f64, t17191: f64, t3566: f64, t3781: f64, t5216: f64, t45618: f64, t43350: f64, t44535: f64, t45607: f64, t13045: f64, t13147: f64, t1770: f64, t1284: f64, t5412: f64, t17306: f64, t3754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59737, t59749, t59788, t59817, t59854, t59864) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2590(t45832, t460, t487, t5219, t5462, t1209, t21451, t17191, t3566, t3781, t5216, t45618);
        let (t59865, t59871, t59872, t59948, t60008, t60019) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2591(t43350, t44535, t45607, t460, t487, t13045, t13147, t1770, t1209, t1284, t5412, t17306, t3754);
    (t59737, t59749, t59788, t59817, t59854, t59864, t59865, t59871, t59872, t59948, t60008, t60019)
}
