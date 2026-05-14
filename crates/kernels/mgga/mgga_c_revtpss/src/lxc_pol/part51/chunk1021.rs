//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1021/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1021<F: Float>(t27619: F, t7150: F, t3268: F, t373: F, t31991: F, t99914: F, t1678: F, t31902: F, t127: F, t31950: F, t33825: F, t371: F, t1028: F, t1032: F, t1042: F, t120263: F, t120479: F, t126600: F, t247: F, t3046: F, t3116: F, t31891: F, t31892: F, t31899: F, t31903: F, t31913: F, t31959: F, t32006: F, t32014: F, t32015: F, t33751: F, t385: F, t4742: F, t4772: F, t4872: F, t4946: F, t5015: F, t8501: F, t8507: F) -> (F,) {
    let t126749 = t7150 * t27619;
    let t126765 = t373 * t3268;
    let t126770 = t99914 * t31991;
    let t126774 = t31902 * t1678 * t31991;
    let t126779 = t31950 * t371 * t127 * t33825;
    let t126786 = -0.17135921299530705785e1 * t31903 * t31892 * t8507 * t4772 + 0.17135921299530705785e1 * t126749 * t31899 + 0.24791552806034007214e-3 * t120263 * t1042 * t4872 * t126600 + 0.56468933516960933998e-3 * t3046 * t1032 * t8501 * t33751 + 0.56468933516960933998e-3 * t31913 * t247 * t3116 * t385 * t4742 + 0.56468933516960933998e-3 * t32014 * t32015 * t126765 * t4946 - 0.5578099381357651623e-3 * t126770 * t32006 + 0.5578099381357651623e-3 * t126774 * t1028 + 0.24791552806034007213e-3 * t126779 - 0.12395776403017003607e-3 * t120479 - 0.17135921299530705785e1 * t31891 * t31959 * t8507 * t5015;
    (t126786,)
}
