//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk770;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk771;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk772;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta135<F: Float>(t2978: F, t974: F, t2770: F, t344: F, t2244: F, t337: F, t39: F, t1887: F, t60: F, t976: F, t984: F, t343: F, t883: F, t607: F) -> (F, F, F, F, F, F, F, F) {
        let t2979 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk770::<F>(t2978, t974);
        let (t2981, t2982, t2986) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk771::<F>(t2770, t344, t2244, t2979, t337, t39, t1887);
        let t2987 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk772::<F>(t60, t976);
        let (t2988, t2989, t2990) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk773::<F>(t2987, t984, t343, t883, t607);
    (t2979, t2981, t2982, t2986, t2987, t2988, t2989, t2990)
}
