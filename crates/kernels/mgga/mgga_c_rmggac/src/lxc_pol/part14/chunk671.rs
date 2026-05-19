//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 671/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk671<F: Float>(t265: F, t570: F, t2079: F, t262: F, t2068: F, t8705: F, t2073: F, t8701: F, t8889: F, t8891: F, t8893: F, t8895: F, t8897: F, t8899: F, t8903: F, t8907: F, t8909: F, t8911: F, t8913: F) -> (F, F) {
    let t8915 = t265 * t570;
    let t8917 = t2079 * t262 * t8915;
    let t8919 = t2068 * t8705;
    let t8921 = t2073 * t8701;
    let t8923 = -F::cast_from(0.20455996240684006296e-1_f64) * t8889 + F::cast_from(0.40911992481368012592e-1_f64) * t8891 + F::cast_from(0.10227998120342003148e-1_f64) * t8893 + F::cast_from(0.40911992481368012592e-1_f64) * t8895 - F::cast_from(0.6818665413561335432e-1_f64) * t8897 - F::cast_from(0.13637330827122670864e-1_f64) * t8899 + F::cast_from(0.10227998120342003148e-1_f64) * t8903 - F::cast_from(0.13637330827122670864e-1_f64) * t8907 - F::cast_from(0.68186654135613354322e-2_f64) * t8909 + F::cast_from(0.27274661654245341728e-1_f64) * t8911 - F::cast_from(0.36366215538993788971e-1_f64) * t8913 - F::cast_from(0.90915538847484472429e-2_f64) * t8917 - F::cast_from(0.10227998120342003148e-1_f64) * t8919 + F::cast_from(0.13637330827122670864e-1_f64) * t8921;
    (t8915, t8923)
}
