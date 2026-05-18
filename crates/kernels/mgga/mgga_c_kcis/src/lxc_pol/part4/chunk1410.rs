//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1410/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1410<F: Float>(t18292: F, t18311: F, t18331: F, t18350: F, t12940: F, t1629: F, t1636: F, t17310: F, t17314: F, t17317: F, t17325: F, t17709: F, t17710: F, t17713: F, t18266: F, t18268: F, t18271: F, t4480: F, t4481: F, t633: F) -> F {
    let t18352 = t18292 + t18311 + t18331 + t18350;
    let t18354 = -F::new(6.0) * t12940 * t17713 - t1629 * t18352 - F::new(2.0) * t1636 * t17710 + t18266 * t633 + F::new(2.0) * t18268 * t4481 + F::new(4.0) * t18271 * t4480 + t17310 + t17314 - t17317 - t17325 + t17709;
    t18354
}
