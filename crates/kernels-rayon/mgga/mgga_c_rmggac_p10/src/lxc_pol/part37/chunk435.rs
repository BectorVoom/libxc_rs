//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 435/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk435(t265: f64, t570: f64, t2079: f64, t262: f64, t2068: f64, t8705: f64, t2073: f64, t8701: f64, t8889: f64, t8891: f64, t8893: f64, t8895: f64, t8897: f64, t8899: f64, t8903: f64, t8907: f64, t8909: f64, t8911: f64, t8913: f64) -> (f64, f64, f64, f64, f64) {
    let t8915 = t265 * t570;
    let t8917 = t2079 * t262 * t8915;
    let t8919 = t2068 * t8705;
    let t8921 = t2073 * t8701;
    let t8923 = -0.20455996240684006296e-1_f64 * t8889 + 0.40911992481368012592e-1_f64 * t8891 + 0.10227998120342003148e-1_f64 * t8893 + 0.40911992481368012592e-1_f64 * t8895 - 0.6818665413561335432e-1_f64 * t8897 - 0.13637330827122670864e-1_f64 * t8899 + 0.10227998120342003148e-1_f64 * t8903 - 0.13637330827122670864e-1_f64 * t8907 - 0.68186654135613354322e-2_f64 * t8909 + 0.27274661654245341728e-1_f64 * t8911 - 0.36366215538993788971e-1_f64 * t8913 - 0.90915538847484472429e-2_f64 * t8917 - 0.10227998120342003148e-1_f64 * t8919 + 0.13637330827122670864e-1_f64 * t8921;
    (t8915, t8917, t8919, t8921, t8923)
}
