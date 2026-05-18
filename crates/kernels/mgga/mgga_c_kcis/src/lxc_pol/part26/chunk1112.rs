//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1112/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1112<F: Float>(t28386: F, t28446: F, t28498: F, t28554: F, t589: F, t1505: F, t8182: F, t1555: F, t2069: F, t27491: F, t27494: F, t5900: F) -> (F, F, F, F, F, F) {
    let t28556 = t28386 + t28446 + t28498 + t28554;
    let t28557 = t28556 * t589;
    let t28558 = t8182 * t1505;
    let t28559 = t28558 * t1555;
    let t28560 = t27491 * t2069;
    let t28562 = F::new(2.0) * t27494 * t5900;
    (t28556, t28557, t28558, t28559, t28560, t28562)
}
