//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1129/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1129<F: Float>(t786: F, t94878: F, t7286: F, t4132: F, t689: F, t7242: F, t2023: F, t4075: F, t9682: F, t26050: F, t26072: F, t213: F, t26034: F, t25899: F, t94664: F, t94404: F) -> (F, F, F, F, F, F, F) {
    let t94894 = t786 * t94878;
    let t94895 = t94894 * t7286;
    let t94898 = t689 * t7242 * t4132;
    let t94901 = t786 * t2023 * t4075;
    let t94902 = t94901 * t9682;
    let t94904 = t26072 * t26050;
    let t94906 = t213 * t26034;
    let t94909 = t25899 * t94664;
    let t94911 = t25899 * t94404;
    (t94895, t94898, t94902, t94904, t94906, t94909, t94911)
}
