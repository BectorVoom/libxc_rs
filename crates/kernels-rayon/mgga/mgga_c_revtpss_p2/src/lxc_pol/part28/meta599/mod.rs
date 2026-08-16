//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2075;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2076;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta599(t25877: f64, t94889: f64, t25881: f64, t786: f64, t94878: f64, t7286: f64, t4132: f64, t689: f64, t7242: f64, t2023: f64, t4075: f64, t9682: f64, t26050: f64, t26072: f64, t213: f64, t26034: f64, t25899: f64, t94664: f64, t94404: f64, t2453: f64, t25949: f64, t25946: f64, t25939: f64, t40270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94890, t94891, t94895, t94898, t94901, t94902) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2075(t25877, t94889, t25881, t786, t94878, t7286, t4132, t689, t7242, t2023, t4075, t9682);
        let (t94904, t94906, t94909, t94911, t94914, t94917) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2076(t26050, t26072, t213, t26034, t25899, t94664, t94404, t2453, t25949, t25946, t25939, t40270);
    (t94890, t94891, t94895, t94898, t94901, t94902, t94904, t94906, t94909, t94911, t94914, t94917)
}
