//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2075;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2076;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta599<F: Float>(t25877: F, t94889: F, t25881: F, t786: F, t94878: F, t7286: F, t4132: F, t689: F, t7242: F, t2023: F, t4075: F, t9682: F, t26050: F, t26072: F, t213: F, t26034: F, t25899: F, t94664: F, t94404: F, t2453: F, t25949: F, t25946: F, t25939: F, t40270: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94890, t94891, t94895, t94898, t94901, t94902) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2075::<F>(t25877, t94889, t25881, t786, t94878, t7286, t4132, t689, t7242, t2023, t4075, t9682);
        let (t94904, t94906, t94909, t94911, t94914, t94917) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2076::<F>(t26050, t26072, t213, t26034, t25899, t94664, t94404, t2453, t25949, t25946, t25939, t40270);
    (t94890, t94891, t94895, t94898, t94901, t94902, t94904, t94906, t94909, t94911, t94914, t94917)
}
