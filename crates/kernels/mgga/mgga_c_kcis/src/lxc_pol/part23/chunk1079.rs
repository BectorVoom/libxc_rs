//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1079/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1079<F: Float>(t2268: F, t4500: F, t2264: F, t4479: F, t4481: F, t1628: F, t7996: F, t12930: F, t12933: F, t12940: F, t1629: F, t1636: F, t27498: F, t27502: F, t27693: F, t4475: F, t4480: F, t7998: F, t8001: F, t8010: F) -> (F, F, F, F, F) {
    let t27697 = t2268 * t4500;
    let t27702 = t2264 * t4479;
    let t27705 = t2268 * t4481;
    let t27710 = t7996 * t1628;
    let t27713 = -t12930 * t2268 + F::cast_from(4.0_f64) * t12933 * t8001 - F::cast_from(6.0_f64) * t12940 * t27705 - t1629 * t27693 - F::cast_from(2.0_f64) * t1636 * t27710 + F::cast_from(2.0_f64) * t27697 * t4480 + F::cast_from(2.0_f64) * t27702 * t4481 - F::cast_from(2.0_f64) * t4475 * t8010 - t4500 * t7998 + t27498 + t27502;
    (t27697, t27702, t27705, t27710, t27713)
}
