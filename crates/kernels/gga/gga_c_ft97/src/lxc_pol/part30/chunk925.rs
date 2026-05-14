//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 925/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk925<F: Float>(t1526: F, t6895: F, t9483: F, t33543: F, t35757: F, t1774: F, t6903: F, t7426: F, t342: F, t35772: F, t630: F, t10915: F, t13616: F, t1403: F, t141478: F, t141489: F, t141491: F, t141509: F, t1424: F, t15567: F, t231: F, t2320: F, t27475: F, t27483: F, t27742: F, t27781: F, t27829: F, t27833: F, t27884: F, t27892: F, t2917: F, t343: F, t3691: F, t3700: F, t461: F, t5996: F, t6141: F, t6745: F, t6900: F, t7150: F, t7427: F) -> (F,) {
    let t151144 = t1526 * t9483 * t6895;
    let t151158 = t35757 * t33543;
    let t151167 = t7426 * t1774 * t6903;
    let t151183 = t342 * t630 * t35772;
    let t151188 = t1526 * t13616 * t27475 / 6.0 - t151144 / 36.0 + t5996 * t6900 / 3.0 + t1403 * t27833 / 3.0 + t1403 * t27884 / 3.0 + t1403 * t27829 / 3.0 - t141478 / 54.0 - t7426 * t461 * t27781 / 6.0 + t151158 / 18.0 - t27892 * t7150 * t7427 / 6.0 - t1526 * t2320 * t27483 / 12.0 + t151167 / 18.0 - t342 * t343 * t231 * t27742 / 4.0 + t15567 * t2917 * t1424 * t3700 / 6.0 - t15567 * t10915 * t1424 * t3691 / 9.0 + t141489 - t141491 / 12.0 - t151183 / 12.0 - t141509 / 9.0 + t6745 * t6141 / 3.0;
    (t151188,)
}
