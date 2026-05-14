//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1056/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1056<F: Float>(t26954: F, t27069: F, t1250: F, t251: F, t34814: F, t7771: F, t92794: F, t2844: F, t3622: F, t3245: F, t7723: F, t10470: F, t2180: F, t7732: F, t26972: F, t7768: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93028 = t27069 * t26954;
    let t93050 = t34814 * t251 * t1250;
    let t93082 = t7771 * t92794;
    let t93089 = t3622 * t2844;
    let t93145 = t3245 * t7723;
    let t93157 = t10470 * t2180;
    let t93158 = 0.51588271604938271604e-3 * t93157;
    let t93163 = t3245 * t7732;
    let t93222 = t7768 * t26972;
    (t93028, t93050, t93082, t93089, t93145, t93157, t93158, t93163, t93222)
}
