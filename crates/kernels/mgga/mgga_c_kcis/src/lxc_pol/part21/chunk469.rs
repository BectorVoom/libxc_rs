//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 469/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk469<F: Float>(t2952: F, t932: F, t2917: F, t242: F, t2944: F, t939: F, t1060: F, t250: F, t253: F, t659: F, t946: F) -> (F, F, F, F, F, F, F, F) {
    let t2953 = t932 * t2952;
    let t2955 = F::cast_from(0.39862222222222222223e0_f64) * t2917;
    let t2960 = F::new(1.0)/F::sqrt(t242);
    let t2961 = t2960 * t2944;
    let t2963 = t939 * t2952;
    let t2966 = t250 * t1060 * t253;
    let t2967 = F::cast_from(0.13692777777777777778e0_f64) * t2966;
    let t2968 = t659 * t946;
    (t2953, t2955, t2960, t2961, t2963, t2966, t2967, t2968)
}
