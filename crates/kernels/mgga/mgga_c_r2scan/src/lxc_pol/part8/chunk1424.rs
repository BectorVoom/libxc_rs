//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1424/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1424<F: Float>(t26306: F, t30804: F, t30807: F, t30827: F, t30840: F, t30844: F, t34479: F, t34481: F, t34483: F, t34485: F, t34487: F, t34492: F, t34496: F, t34500: F, t32444: F, t495: F) -> (F, F) {
    let t34504 = t26306 - 0.17465477326173296717e-1 * t30804 + 0.82318114786693894983e-2 * t30807 - 0.58218257753910989057e-2 * t34479 + 0.10401866088065122276e1 * t34481 + 0.34672886960217074253e0 * t34483 + 0.12805040077930161442e0 * t34485 - 0.11524536070137145298e1 * t34487 + 0.34672886960217074253e0 * t30827 + 0.41607464352260489104e1 * t34492 + 0.11557628986739024751e0 * t34496 - 0.69345773920434148504e0 * t34500 + 0.46098144280548581192e1 * t30840 - 0.23049072140274290596e1 * t30844;
    let t34524 = t32444 * t495;
    (t34504, t34524)
}
