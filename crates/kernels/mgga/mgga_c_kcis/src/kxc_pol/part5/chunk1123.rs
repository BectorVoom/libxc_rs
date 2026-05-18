//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1123/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1123<F: Float>(t13712: F, t13717: F, t13909: F, t13912: F, t13939: F, t18650: F, t18655: F, t18659: F, t18664: F, t18667: F, t18877: F, t18880: F, t18885: F, t18887: F, t18890: F, t18906: F, t18909: F, t18912: F, t18920: F, t18942: F, t9691: F, t9708: F) -> F {
    let t18944 = -F::new(0.13287407407407407408e0) * t9691 - F::new(0.91285185185185185187e-1) * t9708 + F::new(0.18257037037037037037e-1) * t18877 - F::new(0.82156666666666666667e-1) * t18880 - t13909 + F::new(0.36514074074074074073e-1) * t13912 + F::new(0.39862222222222222222e0) * t13717 - F::new(0.76790625e-1) * t18885 + F::new(0.3071625e0) * t18887 + F::new(0.15358125e0) * t18890 + t18906 - F::new(0.36514074074074074075e-1) * t18909 - F::new(0.10954222222222222222e0) * t18912 + F::new(0.13287407407407407407e0) * t13712 - t13939 - F::new(0.33218518518518518518e0) * t18650 + F::new(0.11958666666666666667e1) * t18655 - F::new(0.79724444444444444444e0) * t18659 - F::new(0.17938e1) * t18664 + F::new(0.23917333333333333334e1) * t18667 + F::new(0.3071625e0) * t18920 + t18942;
    t18944
}
