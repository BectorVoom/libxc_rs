//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1219/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1219<F: Float>(t30756: F, t694: F, t226: F, t30612: F, t18514: F, t6035: F, t9665: F, t108447: F, t108448: F, t108487: F, t108781: F, t108794: F, t108972: F, t109231: F, t1095: F, t1113: F, t111831: F, t1120: F, t122987: F, t122990: F, t123006: F, t123015: F, t123028: F, t123035: F, t123039: F, t1614: F, t17828: F, t17836: F, t17864: F, t213: F, t231: F, t232: F, t2347: F, t2360: F, t2393: F, t24265: F, t24389: F, t27500: F, t27552: F, t27595: F, t27616: F, t27617: F, t27618: F, t27646: F, t27651: F, t30598: F, t35410: F, t3766: F, t3886: F, t6019: F, t6024: F, t6034: F, t6043: F, t6046: F, t709: F, t79403: F, t92354: F, t98545: F) -> (F, F) {
    let t123043 = t694 * t30756;
    let t123047 = t30612 * t226;
    let t123056 = t6035 * t9665 * t18514;
    let t123059 = -0.29693535778629056444e-3 * t122987 + 0.29693535778629056444e-3 * t122990 + 0.51074886703703703704e-1 * t27651 * t98545 * t1113 * t2360 * t3886 - 0.34049924469135802469e-1 * t27651 * t108972 * t1113 * t2347 * t3886 + 0.60548059007656442388e-3 * t108447 * t108448 * t111831 * t27646 + 0.51074886703703703704e-1 * t108487 * t98545 * t123006 * t27646 - 0.17816121467177433866e-2 * t109231 * t35410 * t17864 + 0.37454916916049382716e0 * t6043 * t123015 * t6046 + 0.51690243689028715487e-4 * t79403 * t6024 + t108781 + 0.23754828622903245156e-2 * t24265 * t1120 * t27552 + 0.10560293360415908094e-4 * t27616 * t27618 * t27595 * t30598 - 0.6139464401544915801e-7 * t123028 * t92354 * t27617 * t231 * t1095 * t213 + 0.53448364401532301599e-4 * t6034 * t232 * t123035 - 0.89080607335887169332e-4 * t6034 * t232 * t123039 + t108794 - 2.0 * t3766 * t123043 * t709 - 2.0 * t123047 * t6019 + 0.55136259934963963184e-4 * t17836 * t24389 * t1614 * t2393 * t17828 - 0.38306165027777777778e-1 * t27500 * t123056;
    (t123056, t123059)
}
