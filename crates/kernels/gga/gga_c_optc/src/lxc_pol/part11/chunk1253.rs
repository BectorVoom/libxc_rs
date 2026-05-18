//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1253/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1253<F: Float>(t56677: F, t7341: F, t837: F, t845: F, t13796: F, t14091: F, t4919: F, t2472: F, t2476: F, t25183: F, t55901: F, t2633: F, t55906: F) -> (F, F, F, F, F, F) {
    let t56681 = F::new(0.1403573615389248977e2) * t845 * t7341 * t56677 * t837;
    let t56686 = F::new(0.62336721237753107879e3) * t845 * t13796 * t14091;
    let t56689 = t4919 * t4919;
    let t56693 = F::new(0.51947267698127589897e2) * t845 * t2472 * t56689 * t2476;
    let t56700 = t25183 * t55901;
    let t56704 = t2633 * t55906;
    (t56681, t56686, t56689, t56693, t56700, t56704)
}
