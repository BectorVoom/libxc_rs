//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1186/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1186<F: Float>(t2672: F, t56744: F, t1: F, t14635: F, t19: F, t24566: F, t25837: F, t25940: F, t25982: F, t2751: F, t313: F, t323: F, t4962: F, t51515: F, t51564: F, t51577: F, t56700: F, t56732: F, t56844: F, t56848: F, t56867: F, t56897: F, t56902: F, t56908: F, t56911: F, t57541: F, t57628: F, t7857: F, t7870: F, t8208: F, t8209: F, t894: F, t914: F, t930: F, t935: F, t953: F) -> (F, F) {
    let t57770 = t56744 * t2672;
    let t57813 = -0.6237918122117623248e2 * t51515 + 0.1343485452223045261e-1 * t51564 - 0.17386322979577515709e0 * t930 * t914 * t56911 + 0.18014732272771396904e7 * t25982 * t323 * t56897 * t19 - 0.27022098409157095356e7 * t25837 * t323 * t56902 * t19 + 0.69688026546736710315e2 * t2751 * t313 * t57770 * t1 + 0.23184437530160156653e8 * t25940 * t313 * t56897 * t935 - 0.27821325036192187983e8 * t24566 * t313 * t56902 * t935 + 0.80609127133382715661e-1 * t51577 + 0.25190352229182098644e-1 * t953 * t57541 + 0.1343485452223045261e0 * t953 * t894 * t7857 * t57628 + 0.20408653907080965924e7 * t8208 * t14635 * t8209 * t4962 - 0.30228422675018518374e0 * t953 * t894 * t7870 * t57628 + 0.50380704458364197288e-2 * t953 * t56732 + 0.82101888746963877062e-1 * t953 * t56908 + 0.28977204965962526182e-1 * t930 * t914 * t56848 - 0.69545291918310062836e0 * t930 * t914 * t56700 - 0.1209136907000740735e0 * t953 * t56867 + 0.30050434779516693818e0 * t930 * t914 * t56844;
    (t57770, t57813)
}
