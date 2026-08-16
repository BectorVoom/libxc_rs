//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1851;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1852;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta479<F: Float>(t23613: F, t6786: F, t1949: F, t2966: F, t1920: F, t1948: F, t3166: F, t345: F, t6680: F, t6781: F, t6805: F, t968: F, t210: F, t6795: F, t6688: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23614, t23617, t23619, t23620, t23621, t23626, t23628) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1851::<F>(t23613, t6786, t1949, t2966, t1920, t1948, t3166, t345, t6680, t6781, t6805, t968);
        let (t23629, t23631) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1852::<F>(t1920, t23628, t210, t6795);
        let (t23632, t23633) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1853::<F>(t6688, t974, t23631);
    (t23614, t23617, t23619, t23620, t23621, t23626, t23628, t23629, t23631, t23632, t23633)
}
