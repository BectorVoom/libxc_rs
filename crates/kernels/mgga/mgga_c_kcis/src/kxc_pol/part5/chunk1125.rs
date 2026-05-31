//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1125/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1125<F: Float>(t18964: F, t274: F, t1680: F, t4718: F, t2938: F, t6393: F, t949: F, t9660: F, t6390: F, t2988: F, t6389: F, t2986: F) -> (F, F, F, F, F) {
    let t18965 = t18964 * t274;
    let t18968 = t1680 * t4718;
    let t18970 = F::cast_from(4.0_f64) * t2938 * t18968;
    let t18971 = t6393 * t949;
    let t18973 = F::cast_from(0.96490945932906628932e2_f64) * t9660 * t18971;
    let t18974 = t6390 * t949;
    let t18976 = F::cast_from(2.0_f64) * t2938 * t18974;
    let t18977 = t6389 * t2988;
    let t18978 = t18977 * t949;
    let t18980 = F::cast_from(0.16081824322151104822e2_f64) * t2986 * t18978;
    (t18965, t18970, t18973, t18976, t18980)
}
