//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1442/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1442<F: Float>(t122: F, t569: F, t6913: F, t107: F, t1180: F, t2407: F, t10472: F, t10474: F, t10479: F, t10487: F, t10490: F, t10492: F, t14242: F, t14245: F, t16720: F, t17729: F, t17765: F, t18159: F, t18160: F, t18162: F, t18163: F, t18168: F, t18173: F, t18176: F, t18177: F, t18181: F, t18182: F, t18184: F, t18185: F, t18188: F, t18189: F, t18194: F, t18195: F, t18203: F, t18206: F, t18208: F, t18209: F, t18214: F, t18218: F, t18220: F, t18227: F, t18231: F, t18232: F, t18234: F, t18236: F, t18241: F, t18242: F, t18247: F, t18248: F, t18254: F, t18255: F, t18261: F, t18264: F, t18265: F, t18267: F, t18286: F, t18290: F, t18291: F, t18293: F, t18294: F, t18299: F, t18300: F, t18302: F, t18303: F, t18308: F, t18309: F, t18314: F, t18315: F, t18318: F, t18319: F, t18333: F, t18383: F, t18385: F, t18386: F, t18389: F, t18390: F, t18392: F, t18393: F, t202: F, t9066: F, t9070: F) -> F {
    let t18404 = t122 * t569 * t6913;
    let t18407 = t107 * t1180 * t2407;
    let t18410 = F::new(0.3350512821420176) * t14242 - F::new(0.21223777183119583) * t14245 - F::new(8.858140152630787) * t9066 + t9070 - t10472 + F::new(1.328721022894618) * t10474 - t10479 + F::new(0.3891025816905257) * t10487 - F::new(0.053059442957798957) * t10490 - F::new(0.011938374665504766) * t122 * t202 * (t17765 + t17729 + t16720 + t18385 + t18386 + t18314 + t18315 + t18383 + t18318 + t18319 + t18309 + t18308 + t18302 + t18303 + t18299 + t18300 + t18291 + t18293 + t18294 + t18290 + t18286 + t18333 + t18267 + t18264 + t18265 + t18261 + t18254 + t18255 + t18247 + t18248 + t18242 + t18241 + t18236 + t18231 + t18232 + t18234 + t18227 + t18220 + t18218 + t18214 + t18209 + t18206 + t18208 + t18203 + t18195 + t18392 + t18393 + t18389 + t18390 + t18194 + t18188 + t18189 + t18184 + t18185 + t18181 + t18182 + t18177 + t18176 + t18173 + t18168 + t18160 + t18162 + t18163 + t18159) + F::new(0.039794582218349216) * t18404 + F::new(1.328721022894618) * t18407 + F::new(1.0051538464260528) * t10492;
    t18410
}
