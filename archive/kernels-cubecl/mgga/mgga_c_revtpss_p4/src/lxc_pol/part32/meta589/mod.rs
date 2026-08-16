//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1918;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta589<F: Float>(t102972: F, t25411: F, t15003: F, t95773: F, t1579: F, t26550: F, t103005: F, t25375: F, t26506: F, t27216: F, t786: F, t7998: F, t867: F, t2467: F, t1580: F, t26446: F, t689: F, t28368: F, t93321: F, t93374: F, t26544: F, t27213: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t103023, t103030, t103037, t103047, t103063, t103067) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1918::<F>(t102972, t25411, t15003, t95773, t1579, t26550, t103005, t25375, t26506, t27216, t786, t7998, t867);
        let (t103069, t103072, t103086, t103088, t103103, t103114) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1919::<F>(t103067, t2467, t1580, t26446, t689, t28368, t93321, t93374, t26544, t27216, t26506, t27213);
    (t103023, t103030, t103037, t103047, t103063, t103067, t103069, t103072, t103086, t103088, t103103, t103114)
}
