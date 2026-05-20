//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1486/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1486<F: Float>(t221: F, t3829: F, t9921: F, t3978: F, t3970: F, t3989: F, t1388: F, t3934: F, t4002: F, t5671: F, t9828: F, t9832: F, t9837: F, t9842: F, t9847: F, t9893: F, t9896: F, t9901: F, t9906: F, t9910: F, t9914: F, t9919: F) -> (F, F, F, F) {
    let t9923 = t9921 * t221 * t3829;
    let t9924 = t3978 * t9923;
    let t9926 = t3989 * t3970;
    let t9928 = F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t9828 - F::cast_from(0.64311027177104605458e-3_f64) * t3934 * t9832 - F::cast_from(0.51448821741683684367e-2_f64) * t5671 * t9837 + F::cast_from(0.12862205435420921092e-2_f64) * t5671 * t9842 + F::cast_from(0.76230004213927992336e-5_f64) * t9847 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t9893 + F::cast_from(0.30011812682648815881e-2_f64) * t9896 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t9901 - F::cast_from(0.38115002106963996168e-4_f64) * t9906 - F::cast_from(0.17006693853500995666e-1_f64) * t9910 + F::cast_from(0.12862205435420921092e-2_f64) * t4002 * t9914 - F::cast_from(0.60023625365297631762e-2_f64) * t9919 + F::cast_from(0.76230004213927992338e-3_f64) * t9924 + F::cast_from(0.12004725073059526352e-1_f64) * t9926;
    (t9923, t9924, t9926, t9928)
}
