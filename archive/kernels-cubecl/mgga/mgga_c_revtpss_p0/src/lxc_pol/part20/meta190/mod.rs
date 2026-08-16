//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk946;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk947;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta190<F: Float>(t225: F, t9849: F, t9850: F, t9852: F, t9869: F, t4010: F, t73: F, t9400: F, t3889: F, t9737: F, t1394: F, t9628: F, t1392: F, t1395: F, t4045: F, t4050: F, t4053: F, t539: F, t541: F, t5650: F, t543: F, t1390: F, t828: F, t3926: F, t3930: F, t1398: F, t3923: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9872, t9881, t9884, t9887) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk946::<F>(t225, t9849, t9850, t9852, t9869, t4010, t73, t9400, t3889, t9737, t1394, t9628);
        let t9890 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk947::<F>(t1392, t1395, t4045, t4050, t4053, t539, t541, t5650, t9872, t9881, t9884, t9887);
        let (t9891, t9893, t9896, t9898) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk948::<F>(t543, t9890, t1390, t828, t3926, t3930, t1398, t3923);
    (t9872, t9881, t9884, t9887, t9890, t9891, t9893, t9896, t9898)
}
