//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2885/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2885<F: Float>(t40076: F, t40079: F, t40194: F, t40198: F, t77048: F, t77051: F, t77053: F, t77056: F, t77058: F, t77059: F, t77060: F, t14353: F, t18392: F, t18850: F, t18865: F, t18871: F, t1940: F, t198: F, t207: F, t23114: F, t23279: F, t2403: F, t2404: F, t27384: F, t39419: F, t39422: F, t39483: F, t39741: F, t39744: F, t39747: F, t39750: F, t40067: F, t40072: F, t40099: F, t40103: F, t40115: F, t4343: F, t4537: F, t4541: F, t4546: F, t4556: F, t50048: F, t50874: F, t5966: F, t61033: F, t75970: F, t75990: F, t76012: F, t76038: F, t76055: F, t76077: F, t76421: F, t765: F, t76890: F, t76893: F, t76932: F, t76935: F, t76936: F, t76942: F, t76944: F, t76946: F, t76948: F, t76950: F, t76951: F, t76952: F, t76954: F, t76963: F, t76966: F, t76988: F, t76991: F, t76992: F, t76995: F, t76997: F, t76998: F, t77021: F, t77023: F, t77298: F, t77326: F, t77333: F, t77347: F, t77360: F, t77373: F, t77381: F, t77386: F, t77387: F, t77400: F, t77412: F, t77429: F, t77441: F, t77455: F, t77467: F, t775: F, t890: F, t892: F) -> F {
    let t77468 = -t77048 + t77051 + t77053 + t77056 + t77058 + t40076 - t40079 + t40194 + t40198 - t77059 - t77060;
    let t77472 = t77412 - t39483 + t77467 + t77468 + t76963 + t76966 + t77455 + t76946 + t76948 + t39741 + t39744 + t77381 + t76950 + t76951 - t76952 + t76954 + t77400 + t39747 - t40115 + t198 * t207 * (t75970 + t75990 + t76012 + t76038 + t76055 + t76077 + t77298 + t77326) * t892 + F::cast_from(6.0_f64) * t198 * t23114 * t890 * t892 + t76935 + t76936 - t40072 + t76932 + t77347 + t76942 - t76944 + t77429 + t76890 + t76893 + t76988 + t40103 + t76995 + t76997 + t76998 + t76991 + t76992 - t39419 - t39422 + t77021 + t77023 + t40067 + t50874 + t39750 + t77360 - F::cast_from(3.0_f64) * t1940 * t18865 * t4537 + F::cast_from(6.0_f64) * t2403 * t77373 * t775 + F::cast_from(18.0_f64) * t4541 * t2404 * t23279 + F::cast_from(3.0_f64) * t198 * t765 * t76421 + F::cast_from(6.0_f64) * t1940 * t61033 * t27384 - F::cast_from(18.0_f64) * t2403 * t4556 * t77441 + F::cast_from(9.0_f64) * t2403 * t18850 * t4343 + t40099 + F::cast_from(18.0_f64) * t198 * t77333 * t4343 + F::cast_from(9.0_f64) * t2403 * t4546 * t18392 + F::cast_from(6.0_f64) * t1940 * t18871 * t4537 + F::cast_from(18.0_f64) * t4541 * t14353 * t5966 + t50048 + t77386 + t77387;
    t77472
}
