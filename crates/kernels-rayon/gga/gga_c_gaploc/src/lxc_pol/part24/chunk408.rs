//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 408/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk408(t1126: f64, t1138: f64, t1153: f64, t1161: f64, t1825: f64, t1829: f64, t1831: f64, t1882: f64, t1886: f64, t1892: f64, t1897: f64, t1898: f64, t1904: f64, t1908: f64, t1935: f64, t1939: f64, t1944: f64, t1949: f64, t270: f64, t301: f64, t735: f64) -> f64 {
    let t1952 = 0.15381052460284448567e-1_f64 * t270 * t1882 - 0.23071578690426672851e-1_f64 * t270 * t1886 + 0.15381052460284448567e-1_f64 * t270 * t1892 + t1153 - 0.15381052460284448567e-1_f64 * t1897 * t1898 + 0.15381052460284448567e-1_f64 * t1897 * t1904 - t1831 - t1829 + 0.34180116578409885707e-2_f64 * t1908 * t301 + 0.76905262301422242837e-2_f64 * t1935 * t301 + 0.20508069947045931424e-1_f64 * t1939 * t301 - t1126 + t1161 - t1138 + t1825 + 0.8545029144602471425e-3_f64 * t1944 * t735 - 0.17090058289204942853e-2_f64 * t1949 * t735;
    t1952
}
