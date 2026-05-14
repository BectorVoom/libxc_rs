//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1404/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1404<F: Float>(t10197: F, t179: F, t23345: F, t23355: F, t23362: F, t23366: F, t2373: F, t2390: F, t2398: F, t2412: F, t2414: F, t27254: F, t28283: F, t28289: F, t28292: F, t28295: F, t28303: F, t28305: F, t28316: F, t3235: F, t3860: F, t404: F, t758: F, t7988: F, t8418: F) -> (F,) {
    let t28320 = 0.28582678745379824648e-3 * t28283 + 0.72409452821628889107e-2 * t10197 * t2390 + 0.14481890564325777821e-1 * t28289 * t2373 - 0.72409452821628889107e-2 * t28292 * t2398 - 0.30488190661738479624e-2 * t28295 + 0.1270341277572436651e-3 * t23345 - 0.20579528696673473748e-1 * t3235 * t758 * t8418 * t7988 - 0.47637797908966374413e-4 * t28303 + 0.96545937095505185476e-2 * t28305 - 0.67751534803863288053e-3 * t23355 + 0.43445671692977333463e-1 * t3860 * t2414 + 0.25724410870841842184e-2 * t404 * t179 * t2412 * t27254 - 0.28582678745379824649e-3 * t28316 + 0.60976381323476959248e-2 * t23362 + 0.3811023832717309953e-3 * t23366;
    (t28320,)
}
