//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1296/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1296<F: Float>(t104923: F, t22591: F, t554: F, t100932: F, t101013: F, t104637: F, t104671: F, t104857: F, t104888: F, t104897: F, t104901: F, t104912: F, t104917: F, t104920: F, t2043: F, t23711: F, t23847: F, t23869: F, t26692: F, t26745: F, t40223: F, t93169: F, t94508: F, t94524: F, t94530: F, t94535: F, t94547: F, t94549: F, t94553: F) -> (F, F) {
    let t104925 = t22591 * t104923 * t554;
    let t104928 = 0.11853866860905349795e0 * t26692 * t101013 - 0.14817333576131687243e-1 * t104888 - 0.20003400327777777778e0 * t94508 * t93169 * t104637 * t104671 + 0.10741227453659940873e0 * t23711 * t100932 - 0.1611184118048991131e0 * t94535 * t104897 + 0.1611184118048991131e0 * t94524 * t104901 + 0.24163653553615319118e1 * t2043 * t104857 + 0.1611184118048991131e0 * t94530 * t104897 + 0.13335600218518518519e0 * t94547 - 0.1611184118048991131e0 * t94549 - 0.4708574239787593252e-2 * t94553 + 0.90613700826057446696e0 * t40223 * t104912 + 0.48327307107230638238e1 * t23847 * t104917 + 0.90613700826057446696e0 * t104920 * t26745 + 0.90613700826057446696e0 * t23869 * t104925;
    (t104925, t104928)
}
