//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1403/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1403<F: Float>(t17794: F, t3363: F, t1042: F, t372: F, t5268: F, t17695: F, t13086: F, t13090: F, t13092: F, t17693: F, t17781: F, t17786: F, t17791: F, t17792: F, t3640: F, t3644: F, t3711: F, t5331: F, t5381: F) -> F {
    let t17795 = t17794 * t3363;
    let t17796 = t1042 * t17795;
    let t17799 = t372 * t5268;
    let t17800 = t17799 * t17695;
    let t17803 = -F::cast_from(0.14291339372689912324e-3_f64) * t5381 * t3640 - F::cast_from(0.95275595817932748827e-4_f64) * t13086 - F::cast_from(0.19055119163586549765e-3_f64) * t13090 - F::cast_from(0.19055119163586549765e-3_f64) * t13092 - F::cast_from(0.28582678745379824648e-3_f64) * t5381 * t3644 - F::cast_from(0.42874018118069736972e-3_f64) * t5331 * t17781 - F::cast_from(0.21437009059034868486e-3_f64) * t5331 * t17786 - t17791 + t17792 / F::new(162.0) - F::cast_from(0.23818898954483187207e-3_f64) * t3711 * t17796 - F::cast_from(0.57165357490759649296e-3_f64) * t17693 * t17800;
    t17803
}
