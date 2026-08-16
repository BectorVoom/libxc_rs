//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk770;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk771;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta135<F: Float>(t2988: F, t2990: F, t2775: F, t344: F, t2244: F, t977: F, t2250: F, t978: F, t2822: F, t2824: F, t2828: F, t2831: F, t2834: F, t340: F, t343: F, t974: F, t984: F, t2955: F, t2958: F, t2960: F, t2969: F, t2972: F, t2975: F, t2982: F, t2986: F, t346: F, t973: F, t980: F, t987: F, t381: F, t1049: F, t990: F, t225: F, t991: F, t1008: F, t191: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2991, t2995, t2996, t2999, t3000, t3003, t3008) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk770::<F>(t2988, t2990, t2775, t344, t2244, t977, t2250, t978, t2822, t2824, t2828, t2831, t2834);
        let (t3010, t3014, t3016, t3020) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk771::<F>(t3008, t340, t343, t974, t984, t2955, t2958, t2960, t2969, t2972, t2975, t2982, t2986, t2991, t2996, t3000, t346, t973, t980, t987);
        let (t3021, t3023, t3026, t3030) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk772::<F>(t3020, t381, t1049, t990, t225, t991, t1008, t191);
    (t2995, t2999, t3003, t3008, t3010, t3014, t3016, t3020, t3021, t3023, t3026, t3030)
}
