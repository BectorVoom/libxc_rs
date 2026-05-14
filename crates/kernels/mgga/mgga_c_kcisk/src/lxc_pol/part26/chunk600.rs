//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 600/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk600<F: Float>(t2191: F, t425: F, t1175: F, t3564: F, t416: F, t458: F, t1364: F, t2192: F, t1433: F, t5703: F, t457: F, t1216: F, t1421: F, t2110: F, t338: F, t456: F, t5798: F, t5923: F, t5929: F, t5934: F, t5938: F, t5941: F, t5945: F) -> (F, F, F, F, F, F, F, F) {
    let t5948 = t425 * t2191;
    let t5949 = t5948 * t1175;
    let t5950 = t3564 * t5949;
    let t5953 = t416 * t458;
    let t5954 = t2192 * t1364;
    let t5955 = t5953 * t5954;
    let t5958 = t1433 * t5703;
    let t5959 = t457 * t5958;
    let t5966 = 0.98556445e-3 * t1421 * t5923 + 0.7391733375e-3 * t1421 * t5929 - 0.1478346675e-2 * t1421 * t5934 + 0.1478346675e-2 * t456 * t5938 - 0.65704296666666666667e-3 * t5941 - 0.65704296666666666667e-3 * t1421 * t5945 - 0.1478346675e-2 * t1421 * t5950 + 0.19711289e-2 * t1421 * t5955 - 0.98556445e-3 * t456 * t5959 - 4.0 * t1216 * t2110 - 4.0 * t338 * t5798;
    (t5949, t5950, t5953, t5954, t5955, t5958, t5959, t5966)
}
