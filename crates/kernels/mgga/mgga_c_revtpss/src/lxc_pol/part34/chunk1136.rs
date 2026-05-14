//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1136/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1136<F: Float>(t106024: F, t106030: F, t106033: F, t106037: F, t106040: F, t106042: F, t106048: F, t106050: F, t92989: F, t98976: F, t98979: F, t99002: F, t99009: F, t99013: F, t27221: F, t76613: F) -> (F, F) {
    let t113206 = -0.12004725073059526352e0 * t106024 - 0.1084295579938911763e-3 * t98976 + 0.15246000842785598468e-4 * t98979 + 0.81312004494856525162e-3 * t99002 - t92989 - 0.13605355082800796533e0 * t99009 - 0.85748036236139473944e-4 * t106030 + 0.42874018118069736972e-4 * t106033 + 0.32524801797942610064e-2 * t99013 - 0.30492001685571196935e-3 * t106037 + 0.42874018118069736972e-4 * t106040 + 0.60023625365297631762e-2 * t106042 - 0.76230004213927992339e-4 * t106048 + 0.15246000842785598468e-3 * t106050;
    let t113214 = t27221 * t76613;
    (t113206, t113214)
}
