//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1151/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1151<F: Float>(t10817: F, t14386: F, t14388: F, t14393: F, t14395: F, t14399: F, t14401: F, t14406: F, t14408: F, t163: F, t169: F, t171: F, t18765: F, t18779: F, t18782: F, t18786: F, t18788: F, t22013: F, t22048: F, t22082: F, t23185: F, t23191: F, t23192: F, t23194: F, t23195: F, t23198: F, t23199: F, t23201: F, t23202: F, t23206: F, t23208: F, t23210: F, t23211: F, t23214: F, t23215: F, t23218: F, t23219: F, t23224: F, t23225: F, t23227: F, t23228: F, t23231: F, t23238: F, t23240: F, t23242: F, t23246: F, t23247: F, t23249: F, t23250: F, t23253: F, t23255: F, t23257: F, t23258: F, t23268: F, t23269: F, t23271: F, t23275: F, t23279: F, t23280: F, t23282: F, t23289: F, t23290: F, t23293: F, t23294: F, t23296: F, t23297: F, t23302: F, t23304: F, t23307: F, t23308: F, t23311: F, t23312: F, t23314: F, t23315: F, t23323: F, t23324: F, t23326: F, t23330: F, t23333: F, t23334: F, t23340: F, t23342: F) -> (F,) {
    let t23354 = -0.07184540406152766 * t18765 - t14386 - 0.01777850129601853 * t14388 + t14393 + 0.059261670986728444 * t14395 + 0.010403978958430045 * t14399 - 0.07769863529371063 * t14401 - t14406 - 0.004458848125041448 * t14408 - t10817 - 0.001975389032890948 * t23185 - 0.005926167098672845 * t18779 - 0.01185233419734569 * t18782 - 0.0014862827083471494 * t18786 - 0.005388405304614574 * t169 * t171 * (t23279 + t23308 + t23311 + t23312 + t23340 + t23342 + t23323 + t23324 + t23246 + t23240 + t23242 + t22048 + t23333 + t23334 + t23330 + t23297 + t23304 + t23307 + t23253 + t23255 + t23191 + t23192 + t23194 + t23195 + t23214 + t23215 + t23247 + t23249 + t23250 + t23224 + t23225 + t23257 + t23258 + t23210 + t23211 + t23314 + t23315 + t23280 + t23282 + t23293 + t23294 + t23296 + t23268 + t23269 + t23271 + t23302 + t23198 + t23199 + t23201 + t23202 + t23218 + t23219 + t22013 + t23227 + t23228 + t23231 + t23326 + t23238 + t22082 + t23206 + t23208 + t23275 + t23289 + t23290) * t163 - 0.1890324433388467 * t18788;
    (t23354,)
}
