//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1342/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1342<F: Float>(t7601: F, t9446: F, t2201: F, t2687: F, t9268: F, t10315: F, t22709: F, t5108: F, t2551: F, t32532: F, t18786: F, t18839: F, t18843: F, t18855: F, t18869: F, t18872: F, t18875: F, t18878: F, t18888: F, t23320: F, t23321: F, t23694: F, t32071: F, t32078: F, t32087: F, t32088: F) -> (F, F, F, F, F) {
    let t32923 = t7601 * t9446;
    let t32927 = t2201 * t9268 * t2687;
    let t32930 = t5108 * t22709 * t10315;
    let t32936 = t32532 * t2551;
    let t32956 = -t18786 + t32071 - t23320 - t23321 - t18839 + t18843 - t18855 - t23694 - t32078 - t18869 + t18872 + t18875 + t18878 + t32087 + t18888 + t32088;
    (t32923, t32927, t32930, t32936, t32956)
}
