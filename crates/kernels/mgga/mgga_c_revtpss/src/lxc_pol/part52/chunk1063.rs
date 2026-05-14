//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1063/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1063<F: Float>(t1962: F, t4537: F, t4343: F, t119765: F, t119779: F, t126043: F, t126049: F, t126052: F, t126055: F, t126062: F, t126065: F, t126068: F, t126072: F, t126076: F, t126089: F, t119790: F, t121806: F, t121810: F, t121815: F, t126081: F, t126083: F, t126085: F, t126087: F, t126095: F, t1955: F, t1959: F, t28340: F) -> (F, F, F, F) {
    let t127593 = t1962 * t4537;
    let t127596 = t1962 * t4343;
    let t127615 = -0.34708173928447610099e-2 * t126043 - t119765 + 0.225875734067843736e-2 * t126049 - 0.29749863367240808656e-2 * t126052 - 0.22312397525430606492e-2 * t126055 - t119779 - 0.22312397525430606492e-2 * t126062 - 0.29749863367240808656e-2 * t126065 + 0.7437465841810202164e-3 * t126068 - 0.14874931683620404328e-2 * t126072 - 0.14874931683620404328e-2 * t126076;
    let t127620 = 0.13223814266738539448e-3 * t126089;
    let t127628 = -0.29749863367240808656e-2 * t126081 + 0.7437465841810202164e-3 * t126083 + 0.7437465841810202164e-3 * t126085 - 0.74374658418102021639e-4 * t126087 + t127620 - 0.25702851531048074406e-1 * t121806 - 0.8673628188205199462e0 * t1955 * t28340 * t1959 + 0.28559868832551176308e-1 * t121810 + t119790 + 0.14456046980341999104e-1 * t121815 + 0.56468933516960933999e-3 * t126095;
    (t127593, t127596, t127615, t127628)
}
