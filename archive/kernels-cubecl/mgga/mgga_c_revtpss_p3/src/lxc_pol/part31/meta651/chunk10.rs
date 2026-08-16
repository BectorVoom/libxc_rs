//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2162/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2162<F: Float>(t107225: F, t11249: F, t4746: F, t7810: F, t29834: F, t7143: F, t1000: F, t100698: F, t100737: F, t1071: F, t1096: F, t19452: F, t1985: F, t1986: F, t25476: F, t27441: F, t27609: F, t27696: F, t29740: F, t29748: F, t29843: F, t29875: F, t4742: F, t6250: F, t7144: F, t7145: F, t7160: F, t7162: F, t93498: F, t93502: F, t93921: F, t94042: F, t988: F, t99743: F, t99824: F, t999: F) -> (F, F) {
    let t107268 = t107225 * t11249;
    let t107283 = t4746 * t7810;
    let t107286 = t29834 * t7143;
    let t107305 = -F::cast_from(0.17347256376410398924e1_f64) * t25476 * t29748 - F::cast_from(0.17347256376410398924e1_f64) * t7144 * t7145 * t7810 * t4742 + F::cast_from(0.34694512752820797848e1_f64) * t93502 * t29843 * t93498 - F::cast_from(0.4336814094102599731e0_f64) * t99743 * t107268 * t19452 - F::cast_from(0.17347256376410398924e1_f64) * t100737 * t29740 - F::cast_from(0.17347256376410398924e1_f64) * t94042 * t29740 - F::cast_from(0.8673628188205199462e0_f64) * t29834 * t1071 * t1986 + F::cast_from(0.17347256376410398924e1_f64) * t27609 * t27441 - F::cast_from(0.52041769129231196772e1_f64) * t27609 * t27696 - F::cast_from(0.13170898365871023197e1_f64) * t107283 * t1000 + F::cast_from(0.17347256376410398924e1_f64) * t107286 * t7162 + F::cast_from(0.17347256376410398924e1_f64) * t7144 * t7160 * t29875 * t1096 - F::cast_from(0.69389025505641595696e1_f64) * t93921 * t1985 * t6250 * t988 + F::cast_from(0.10408353825846239354e2_f64) * t99824 * t1985 * t6250 * t999 + F::cast_from(0.10408353825846239354e2_f64) * t100698 * t1985 * t6250 * t1096;
    (t107268, t107305)
}
