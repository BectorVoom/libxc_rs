//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1400/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1400<F: Float>(t20919: F, t9848: F, t113573: F, t6244: F, t79107: F, t9486: F, t14294: F, t6394: F, t9831: F, t119881: F, t119883: F, t119885: F, t119886: F, t120006: F, t120023: F, t120060: F, t120087: F, t120111: F, t120125: F, t120171: F, t120201: F, t120223: F, t120252: F, t120279: F, t120301: F, t120316: F, t120341: F, t120367: F, t120388: F, t120409: F, t120435: F, t120464: F, t120489: F, t120517: F, t120536: F, t120553: F, t120576: F, t120609: F, t120647: F, t120670: F, t120683: F, t120708: F, t120735: F, t120760: F, t120789: F, t120818: F, t15094: F, t1620: F, t2748: F, t28049: F, t28136: F, t32523: F, t32533: F, t33757: F, t34909: F, t34912: F, t4535: F, t555: F, t57164: F, t6638: F, t8436: F, t8455: F, t9571: F, t9891: F) -> (F, F, F, F, F) {
    let t120829 = 2.0 * t20919 * t9848;
    let t120836 = 4.0 * t113573 * t6244;
    let t120841 = 2.0 * t79107 * t9486;
    let t120844 = 12.0 * t14294 * t9831 * t6394;
    let t120849 = -t119881 + t119883 + 4.0 * t32523 * t28049 - t119885 + t119886 - 6.0 * t15094 * t34912 * t1620 - t120006 + (t120171 + t120536 + t120683 + t120367 + t120223 + t120252 + t120060 + t120609 + t120023 + t120489 + t120517 + t120647 + t120087 + t120553 + t120818 + t120301 + t120388 + t120464 + t120409 + t120111 + t120670 + t120789 + t120760 + t120435 + t120708 + t120735 + t120341 + t120576 + t120316 + t120279 + t120201 + t120125) * t555 - 6.0 * t15094 * t9571 * t8436 + t120829 - 12.0 * t15094 * t34909 * t1620 - 12.0 * t57164 * t33757 - t120836 + 4.0 * t4535 * t9891 * t6638 - t120841 + t120844 + 2.0 * t4535 * t2748 * t28136 - t32533 * t8455;
    (t120829, t120836, t120841, t120844, t120849)
}
