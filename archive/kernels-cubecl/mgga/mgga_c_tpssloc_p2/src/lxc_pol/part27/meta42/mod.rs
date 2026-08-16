//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta42 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk295;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk296;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk297;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk298;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk299;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk300;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk301;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk302;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk303;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk304;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta42<F: Float>(t240: F, t815: F, t812: F, t241: F, t244: F, t67: F, t120: F, t246: F, t225: F, t680: F, t705: F, t710: F, t719: F, t752: F, t755: F, t760: F, t765: F, t68: F, t776: F, t228: F, t230: F, t232: F, t590: F, t61: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t816 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk295::<F>(t240, t815);
        let t817 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk296::<F>(t812, t816);
        let t819 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk297::<F>(t241, t244, t67);
        let t820 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk298::<F>(t120, t246);
        let t822 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk299::<F>(t225, t680, t705, t710, t719, t752, t755, t760, t765);
        let t824 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk300::<F>(t244, t68);
        let (t825, t828) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk301::<F>(t776, t824, t228, t230, t822);
        let t829 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk302::<F>(t232, t828);
        let t831 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk303::<F>(t819, t820, t829);
        let t835 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk304::<F>(t590, t61);
        let t836 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk305::<F>(t241, t835);
    (t816, t817, t819, t820, t822, t824, t825, t828, t829, t831, t835, t836)
}
