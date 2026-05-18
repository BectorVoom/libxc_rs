//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 622/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk622<F: Float>(t4839: F, t1218: F, t156: F, t1447: F, t1392: F, t1396: F, t4: F, t427: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4840 = F::new(0.21687161765563048428e-1) * t4839;
    let t4841 = t156 * t1218;
    let t4842 = t1447 * t4841;
    let t4843 = F::new(0.32530742648344572643e-1) * t4842;
    let t4844 = t156 * t1392;
    let t4845 = t1447 * t4844;
    let t4846 = F::new(0.48159446095139119799e0) * t4845;
    let t4847 = t156 * t1396;
    let t4848 = t1447 * t4847;
    let t4849 = F::new(0.16265371324172286321e-1) * t4848;
    let t4850 = t427 * t4;
    (t4840, t4841, t4842, t4843, t4844, t4845, t4846, t4847, t4848, t4849, t4850)
}
