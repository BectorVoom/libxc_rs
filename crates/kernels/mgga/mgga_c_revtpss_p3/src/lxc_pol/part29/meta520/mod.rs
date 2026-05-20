//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1842;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta520<F: Float>(t2482: F, t7262: F, t814: F, t9821: F, t820: F, t844: F, t3940: F, t596: F, t7269: F, t3981: F, t25986: F, t2661: F, t9930: F, t25981: F, t843: F, t4006: F, t2681: F, t1401: F, t25997: F, t9905: F, t533: F, t816: F, t92993: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94423, t94424, t94429, t94430, t94443, t94444, t94449) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1842::<F>(t2482, t7262, t814, t9821, t820, t844, t3940, t596, t7269, t3981, t25986, t2661, t9930);
        let (t94456, t94459, t94460, t94468, t94471) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1843::<F>(t25981, t820, t843, t4006, t2681, t7262, t1401, t25997, t9905, t533, t816, t92993);
    (t94423, t94424, t94429, t94430, t94443, t94444, t94449, t94456, t94459, t94460, t94468, t94471)
}
