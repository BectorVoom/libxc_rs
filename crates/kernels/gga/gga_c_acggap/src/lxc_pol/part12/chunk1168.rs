//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1168/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1168<F: Float>(t34340: F, t34347: F, t30330: F, t30334: F, t30343: F, t30347: F, t30349: F, t30353: F, t30355: F, t30362: F, t32435: F, t34327: F, t34330: F, t34336: F, t34343: F, t34349: F, t34351: F) -> F {
    let t37034 = F::cast_from(0.13719685797782315831e-1_f64) * t34340;
    let t37036 = F::cast_from(0.28582678745379824648e-3_f64) * t34347;
    let t37043 = F::new(0.22921875e-1) * t34327 + F::new(0.4584375e-1) * t34330 - F::cast_from(0.42874018118069736972e-2_f64) * t30330 - F::cast_from(0.17149607247227894789e-2_f64) * t30334 + t32435 + F::cast_from(0.62896184579208304138e-3_f64) * t34336 + F::cast_from(0.21437009059034868486e-2_f64) * t30343 + F::cast_from(0.85748036236139473944e-3_f64) * t30347 + t37034 - F::cast_from(0.21437009059034868486e-3_f64) * t34343 - t37036 - F::cast_from(0.75475421495049964966e-2_f64) * t34349 + F::cast_from(0.75475421495049964966e-2_f64) * t34351 - F::cast_from(0.18868855373762491241e-2_f64) * t30349 + F::cast_from(0.20965394859736101379e-3_f64) * t30353 - F::cast_from(0.21437009059034868486e-3_f64) * t30355 - F::cast_from(0.14291339372689912324e-3_f64) * t30362;
    t37043
}
