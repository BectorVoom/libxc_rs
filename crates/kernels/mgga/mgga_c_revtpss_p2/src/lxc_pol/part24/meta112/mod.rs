//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk634;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk635;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk636;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta112<F: Float>(t159: F, t550: F, t216: F, t1376: F, t2689: F, t2700: F, t535: F, t1369: F, t794: F, t2453: F, t546: F, t1389: F, t2713: F, t2668: F, t816: F, t1379: F, t1408: F, t2482: F, t27: F, t136: F, t1413: F, t247: F, t2682: F, t548: F, t820: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3943, t3944, t3950, t3956, t3957, t3964) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk634::<F>(t159, t550, t216, t1376, t2689, t2700, t535, t1369, t794, t2453, t546);
        let (t3967, t3976, t3978) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk635::<F>(t1389, t2713, t3964, t2668, t550, t816, t1379, t1408, t2482, t27);
        let (t3979, t3987, t3989) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk636::<F>(t136, t1413, t247, t2682, t550, t548, t1408, t820, t843);
    (t3943, t3944, t3950, t3956, t3957, t3964, t3967, t3976, t3978, t3979, t3987, t3989)
}
