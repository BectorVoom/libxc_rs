//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 596/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk596<F: Float>(t2073: F, t8713: F, t265: F, t570: F, t2079: F, t262: F, t2068: F, t8705: F, t8701: F, t8889: F, t8891: F, t8893: F, t8895: F, t8897: F, t8899: F, t8903: F, t8907: F, t8909: F, t8911: F) -> (F, F, F, F) {
    let t8913 = t2073 * t8713;
    let t8915 = t265 * t570;
    let t8917 = t2079 * t262 * t8915;
    let t8919 = t2068 * t8705;
    let t8921 = t2073 * t8701;
    let t8923 = -0.20455996240684006296e-1 * t8889 + 0.40911992481368012592e-1 * t8891 + 0.10227998120342003148e-1 * t8893 + 0.40911992481368012592e-1 * t8895 - 0.6818665413561335432e-1 * t8897 - 0.13637330827122670864e-1 * t8899 + 0.10227998120342003148e-1 * t8903 - 0.13637330827122670864e-1 * t8907 - 0.68186654135613354322e-2 * t8909 + 0.27274661654245341728e-1 * t8911 - 0.36366215538993788971e-1 * t8913 - 0.90915538847484472429e-2 * t8917 - 0.10227998120342003148e-1 * t8919 + 0.13637330827122670864e-1 * t8921;
    (t8913, t8915, t8917, t8923)
}
