//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1848;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta523<F: Float>(t25240: F, t3951: F, t3964: F, t25972: F, t9761: F, t2681: F, t7269: F, t820: F, t1416: F, t25978: F, t3970: F, t240: F, t25981: F, t2661: F, t9935: F, t25987: F, t9775: F, t25986: F, t9769: F, t4014: F, t9923: F, t2453: F, t4086: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94540, t94542, t94545, t94546, t94548, t94550) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1848::<F>(t25240, t3951, t3964, t25972, t9761, t2681, t7269, t820, t1416, t25978, t3970, t240, t25981);
        let (t94552, t94554, t94557, t94559, t94561, t94564) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1849::<F>(t2661, t94550, t9935, t25987, t9775, t25986, t9769, t25978, t4014, t25972, t9923, t2453, t4086, t64);
    (t94540, t94542, t94545, t94546, t94548, t94550, t94552, t94554, t94557, t94559, t94561, t94564)
}
