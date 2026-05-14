//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1058/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1058<F: Float>(t1904: F, t717: F, t1138: F, t1597: F, t2916: F, t5466: F, t10808: F, t10812: F, t10816: F, t10817: F, t11667: F, t11670: F, t11904: F, t11933: F, t11959: F, t11993: F, t12029: F, t12060: F, t12099: F, t12131: F, t12174: F, t12208: F, t12250: F, t12286: F, t12332: F, t12377: F, t12424: F, t12466: F, t12506: F, t12539: F, t12576: F, t12618: F, t12651: F, t12678: F, t12716: F, t12736: F, t12755: F, t12785: F, t12825: F, t12864: F, t12898: F, t12933: F, t12967: F, t13002: F, t13040: F, t13075: F, t13121: F, t13155: F, t13199: F, t13235: F, t13277: F, t13320: F, t13362: F, t13393: F, t13414: F, t13448: F, t13484: F, t13519: F, t13553: F, t13756: F, t13793: F, t13880: F, t13912: F, t13946: F, t13982: F, t13996: F, t14026: F, t14059: F, t14102: F, t14184: F, t14228: F, t14265: F, t14296: F, t14304: F, t14341: F, t14369: F, t14382: F, t14386: F, t14388: F, t14393: F, t14395: F, t14399: F, t14401: F, t163: F, t169: F, t171: F) -> (F, F) {
    let t14403 = t717 * t1904;
    let t14405 = t14403 * t1138 * t1597;
    let t14406 = 0.0014862827083471494 * t14405;
    let t14408 = t5466 * t2916 * t1597;
    let t14410 = 0.059261670986728444 * t10808 + 0.010403978958430045 * t10812 + t10816 - t11667 + 0.02694202652307287 * t11670 - 0.005388405304614574 * t169 * t171 * (t14296 + t12825 + t12898 + t12099 + t12377 + t12539 + t12208 + t14304 + t12755 + t13320 + t12736 + t12250 + t11959 + t12060 + t11904 + t12466 + t12332 + t14026 + t14228 + t14369 + t13155 + t12678 + t14265 + t13277 + t13414 + t12864 + t12967 + t14059 + t12785 + t13519 + t12286 + t13199 + t12716 + t13946 + t12029 + t13121 + t12131 + t13075 + t14102 + t14184 + t13448 + t13756 + t12506 + t13793 + t13040 + t12174 + t14341 + t12618 + t12576 + t13553 + t13912 + t13362 + t13393 + t13996 + t13880 + t13002 + t13982 + t11933 + t13484 + t12651 + t13235 + t12933 + t12424 + t11993) * t163 - 0.005926167098672845 * t14382 - t14386 - 0.005926167098672845 * t14388 + t14393 + 0.01975389032890948 * t14395 + 0.0034679929861433484 * t14399 - 0.025899545097903542 * t14401 - t14406 - 0.0014862827083471494 * t14408 - t10817;
    (t14403, t14410)
}
