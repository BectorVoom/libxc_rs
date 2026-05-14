//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 394/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk394<F: Float>(t1126: F, t1138: F, t1153: F, t1161: F, t1825: F, t1829: F, t1831: F, t1882: F, t1886: F, t1892: F, t1897: F, t1898: F, t1904: F, t1908: F, t1935: F, t1939: F, t1944: F, t1949: F, t270: F, t301: F, t735: F) -> (F,) {
    let t1952 = 0.15381052460284448567e-1 * t270 * t1882 - 0.23071578690426672851e-1 * t270 * t1886 + 0.15381052460284448567e-1 * t270 * t1892 + t1153 - 0.15381052460284448567e-1 * t1897 * t1898 + 0.15381052460284448567e-1 * t1897 * t1904 - t1831 - t1829 + 0.34180116578409885707e-2 * t1908 * t301 + 0.76905262301422242837e-2 * t1935 * t301 + 0.20508069947045931424e-1 * t1939 * t301 - t1126 + t1161 - t1138 + t1825 + 0.8545029144602471425e-3 * t1944 * t735 - 0.17090058289204942853e-2 * t1949 * t735;
    (t1952,)
}
