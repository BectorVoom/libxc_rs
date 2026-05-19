//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1008/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1008<F: Float>(t11863: F, t11865: F, t11867: F, t11870: F, t11873: F, t11879: F, t11882: F, t11885: F, t11890: F, t11893: F, t11895: F, t11898: F, t11900: F, t11903: F, t11906: F, t11908: F, t11911: F, t11914: F, t11919: F) -> F {
    let t12547 = -F::cast_from(0.40481770833333333335e-4_f64) * t11863 - F::cast_from(0.40481770833333333335e-4_f64) * t11865 + F::cast_from(0.21642471925239962897e-3_f64) * t11867 + F::cast_from(0.3077456993052877797e-8_f64) * t11870 + F::cast_from(0.67528199161846004231e-6_f64) * t11873 + F::cast_from(0.20220636637604418766e-5_f64) * t11879 - F::cast_from(0.18115908419564701085e-6_f64) * t11882 - F::cast_from(0.10567613244746075633e-6_f64) * t11885 + F::cast_from(0.63350674672043801542e-5_f64) * t11890 - F::cast_from(0.67528199161846004231e-6_f64) * t11893 - F::cast_from(0.67528199161846004231e-6_f64) * t11895 - F::cast_from(0.40021712703254065175e-7_f64) * t11898 - F::cast_from(0.40094868252346065012e-6_f64) * t11900 + F::cast_from(0.18115908419564701085e-6_f64) * t11903 - F::cast_from(0.3623181683912940217e-6_f64) * t11906 + F::cast_from(0.3623181683912940217e-6_f64) * t11908 - F::cast_from(0.21102562238076876322e-7_f64) * t11911 - F::cast_from(0.42205124476153752644e-7_f64) * t11914 + F::cast_from(0.7858244913289017243e-8_f64) * t11919;
    t12547
}
