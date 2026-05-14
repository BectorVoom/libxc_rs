//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1462/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1462<F: Float>(t19712: F, t20180: F, t25041: F, t32999: F, t33000: F, t33001: F, t33002: F, t33003: F, t34900: F, t34903: F, t34907: F, t10397: F, t10549: F, t18777: F, t18786: F, t18839: F, t18843: F, t18855: F, t23214: F, t23215: F, t23216: F, t23218: F, t23219: F, t23320: F, t23321: F, t23694: F, t32071: F, t32075: F, t32089: F, t34915: F, t34916: F, t35252: F, t35253: F, t35255: F, t35258: F, t35264: F, t35265: F, t35267: F, t35268: F, t35271: F, t35272: F, t35275: F, t4899: F, t4969: F, t8: F, t8549: F, t8551: F, t8554: F, t885: F) -> (F,) {
    let t35276 = t25041 - t19712 - t34900 - t34903 + t34907 - t32999 - t20180 - t33000 - t33001 - t33002 + t33003;
    let t35282 = t4899 - 6.0 * t8549 - 0.35089341735807877242e1 * t8551 + t10397 + t18777 - 0.10986868383603927032e-2 * t8554 + t8 * (t10549 * t885 - t18786 - t18839 + t18843 - t18855 - t23320 - t23321 - t23694 + t32071 - t32075 + t32089 + t34915 + t34916 + t35252 + t35253 + t35255 + t35258 + t35264 + t35265 + t35267 + t35268 + t35271 + t35272 + t35275 + t35276) + t23214 - t23215 + t23216 + t4969 + t23218 - t23219;
    (t35282,)
}
