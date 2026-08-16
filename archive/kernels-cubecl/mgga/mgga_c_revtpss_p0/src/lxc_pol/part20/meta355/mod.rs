//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1295;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1296;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1297;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta355<F: Float>(t2566: F, t701: F, t9274: F, t2584: F, t9311: F, t676: F, t9387: F, t2629: F, t9372: F, t2434: F, t2516: F, t8779: F, t9645: F, t252: F, t685: F, t788: F, t10115: F, t862: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39525, t39528) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1295::<F>(t2566, t701, t9274);
        let t39531 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1296::<F>(t2584, t39525, t9311);
        let (t39532, t39534, t39535, t39537, t39538, t39540, t39545, t39549, t39550) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1297::<F>(t676, t9387, t2629, t9372, t2434, t2516, t8779, t9645, t252, t685, t788, t10115, t862);
    (t39525, t39528, t39531, t39532, t39534, t39535, t39537, t39538, t39540, t39545, t39549, t39550)
}
