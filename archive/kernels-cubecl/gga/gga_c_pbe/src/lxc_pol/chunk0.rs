//! GGA_C_PBE lxc pol — lxc_pol chunk-first struct-interface chunk 0/5.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[derive(CubeType)]
pub struct Chunk0Out<F: Float> {
    pub t1: F,
    pub t2: F,
    pub t3: F,
    pub t4: F,
    pub t5: F,
    pub t6: F,
    pub t7: F,
    pub t8: F,
    pub t11: F,
    pub t13: F,
    pub t14: F,
    pub t19: F,
    pub t21: F,
    pub t22: F,
    pub t25: F,
    pub t27: F,
    pub t30: F,
    pub t31: F,
    pub t34: F,
    pub t35: F,
    pub t36: F,
    pub t38: F,
    pub t39: F,
    pub t40: F,
    pub t41: F,
    pub t43: F,
    pub t47: F,
    pub t50: F,
    pub t52: F,
    pub t55: F,
    pub t56: F,
    pub t59: F,
    pub t60: F,
    pub t62: F,
    pub t67: F,
    pub t70: F,
    pub t71: F,
    pub t75: F,
    pub t80: F,
    pub t83: F,
    pub t84: F,
    pub t85: F,
    pub t87: F,
    pub t88: F,
    pub t93: F,
    pub t95: F,
    pub t98: F,
    pub t99: F,
    pub t100: F,
    pub t101: F,
    pub t103: F,
    pub t105: F,
    pub t106: F,
    pub t108: F,
    pub t111: F,
    pub t112: F,
    pub t115: F,
    pub t116: F,
    pub t118: F,
    pub t119: F,
    pub t121: F,
    pub t122: F,
    pub t123: F,
    pub t125: F,
    pub t127: F,
    pub t129: F,
    pub t130: F,
    pub t131: F,
    pub t132: F,
    pub t133: F,
    pub t134: F,
    pub t135: F,
    pub t136: F,
    pub t137: F,
    pub t138: F,
    pub t141: F,
    pub t142: F,
    pub t143: F,
    pub t146: F,
    pub t147: F,
    pub t148: F,
    pub t150: F,
    pub t151: F,
    pub t154: F,
    pub t155: F,
    pub t159: F,
    pub t160: F,
    pub t161: F,
    pub t163: F,
    pub t164: F,
    pub t168: F,
    pub t171: F,
    pub t179: F,
    pub t180: F,
    pub t181: F,
    pub t184: F,
    pub t185: F,
    pub t188: F,
    pub t189: F,
    pub t190: F,
    pub t195: F,
    pub t199: F,
    pub t204: F,
    pub t205: F,
    pub t210: F,
    pub t211: F,
    pub t212: F,
    pub t217: F,
    pub t218: F,
    pub t219: F,
    pub t225: F,
    pub t226: F,
    pub t227: F,
    pub t232: F,
    pub t233: F,
    pub t234: F,
    pub t237: F,
    pub t238: F,
    pub t242: F,
    pub t244: F,
    pub t247: F,
    pub t249: F,
    pub t252: F,
    pub t253: F,
    pub t257: F,
    pub t262: F,
    pub t263: F,
    pub t266: F,
    pub t268: F,
    pub t269: F,
    pub t273: F,
    pub t274: F,
    pub t275: F,
    pub t276: F,
    pub t277: F,
    pub t280: F,
    pub t281: F,
    pub t282: F,
    pub t283: F,
    pub t285: F,
    pub t286: F,
    pub t287: F,
    pub t288: F,
    pub t290: F,
    pub t292: F,
    pub t295: F,
    pub t296: F,
    pub t297: F,
    pub t298: F,
    pub t302: F,
    pub t303: F,
    pub t304: F,
    pub t305: F,
    pub t308: F,
    pub t310: F,
    pub t312: F,
    pub t313: F,
    pub t314: F,
    pub t316: F,
    pub t319: F,
    pub t320: F,
    pub t322: F,
    pub t323: F,
    pub t324: F,
    pub t325: F,
    pub t326: F,
    pub t331: F,
    pub t332: F,
    pub t334: F,
    pub t335: F,
    pub t336: F,
    pub t340: F,
    pub t344: F,
    pub t349: F,
    pub t350: F,
    pub t351: F,
    pub t352: F,
    pub t361: F,
    pub t362: F,
    pub t363: F,
    pub t365: F,
    pub t366: F,
    pub t370: F,
    pub t372: F,
    pub t375: F,
    pub t376: F,
    pub t377: F,
    pub t378: F,
    pub t382: F,
    pub t385: F,
    pub t386: F,
    pub t388: F,
    pub t393: F,
    pub t394: F,
    pub t396: F,
    pub t397: F,
    pub t398: F,
    pub t401: F,
    pub t402: F,
    pub t404: F,
    pub t408: F,
    pub t411: F,
    pub t412: F,
    pub t414: F,
    pub t415: F,
    pub t416: F,
    pub t417: F,
    pub t418: F,
    pub t419: F,
    pub t420: F,
    pub t422: F,
    pub t427: F,
    pub t428: F,
    pub t430: F,
    pub t432: F,
    pub t433: F,
    pub t435: F,
    pub t436: F,
    pub t437: F,
    pub t438: F,
    pub t440: F,
    pub t442: F,
    pub t443: F,
    pub t448: F,
    pub t449: F,
    pub t458: F,
    pub t462: F,
    pub t470: F,
    pub t471: F,
    pub t472: F,
    pub t473: F,
    pub t474: F,
    pub t475: F,
    pub t476: F,
    pub t477: F,
    pub t478: F,
    pub t479: F,
    pub t480: F,
    pub t481: F,
    pub t482: F,
    pub t483: F,
    pub t484: F,
    pub t485: F,
    pub t486: F,
    pub t487: F,
    pub t488: F,
    pub t489: F,
    pub t490: F,
    pub t491: F,
    pub t492: F,
    pub t493: F,
    pub t494: F,
    pub t495: F,
    pub t496: F,
    pub t497: F,
    pub t504: F,
    pub t506: F,
    pub t507: F,
    pub t508: F,
    pub t509: F,
    pub t510: F,
    pub t511: F,
    pub t512: F,
    pub t513: F,
    pub t516: F,
    pub t517: F,
    pub t519: F,
    pub t524: F,
    pub t525: F,
    pub t528: F,
    pub t534: F,
    pub t535: F,
    pub t536: F,
    pub t537: F,
    pub t538: F,
    pub t540: F,
    pub t541: F,
    pub t542: F,
    pub t543: F,
    pub t544: F,
    pub t546: F,
    pub t547: F,
    pub t548: F,
    pub t549: F,
    pub t550: F,
    pub t551: F,
    pub t552: F,
    pub t553: F,
    pub t554: F,
    pub t556: F,
    pub t557: F,
    pub t558: F,
    pub t559: F,
    pub t560: F,
    pub t562: F,
    pub t563: F,
    pub t564: F,
    pub t571: F,
    pub t579: F,
    pub t580: F,
    pub t581: F,
    pub t582: F,
    pub t585: F,
    pub t586: F,
    pub t589: F,
    pub t590: F,
    pub t593: F,
    pub t594: F,
    pub t595: F,
    pub t596: F,
    pub t600: F,
    pub t604: F,
    pub t605: F,
    pub t607: F,
    pub t608: F,
    pub t609: F,
    pub t610: F,
    pub t612: F,
    pub t616: F,
    pub t620: F,
    pub t621: F,
    pub t622: F,
    pub t623: F,
    pub t624: F,
    pub t625: F,
    pub t626: F,
    pub t632: F,
    pub t636: F,
    pub t637: F,
    pub t638: F,
    pub t639: F,
    pub t640: F,
    pub t649: F,
    pub t650: F,
    pub t653: F,
    pub t654: F,
    pub t655: F,
    pub t656: F,
    pub t657: F,
    pub t658: F,
    pub t662: F,
    pub t663: F,
    pub t667: F,
    pub t671: F,
    pub t672: F,
    pub t675: F,
    pub t678: F,
    pub t679: F,
    pub t682: F,
    pub t683: F,
    pub t684: F,
    pub t685: F,
    pub t686: F,
    pub t687: F,
    pub t688: F,
    pub t689: F,
    pub t690: F,
    pub t691: F,
    pub t692: F,
    pub t693: F,
    pub t694: F,
    pub t695: F,
    pub t696: F,
    pub t697: F,
    pub t699: F,
    pub t700: F,
    pub t701: F,
    pub t702: F,
    pub t703: F,
    pub t704: F,
    pub t705: F,
    pub t706: F,
    pub t707: F,
    pub t708: F,
    pub t709: F,
    pub t712: F,
    pub t716: F,
    pub t719: F,
    pub t722: F,
    pub t723: F,
    pub t725: F,
    pub t728: F,
    pub t730: F,
    pub t734: F,
    pub t735: F,
    pub t737: F,
    pub t739: F,
    pub t741: F,
    pub t745: F,
    pub t746: F,
    pub t747: F,
    pub t749: F,
    pub t753: F,
    pub t756: F,
    pub t757: F,
    pub t762: F,
    pub t763: F,
    pub t764: F,
    pub t765: F,
    pub t768: F,
    pub t769: F,
    pub t773: F,
    pub t777: F,
    pub t784: F,
    pub t785: F,
    pub t787: F,
    pub t788: F,
    pub t789: F,
    pub t790: F,
    pub t791: F,
    pub t792: F,
    pub t793: F,
    pub t794: F,
    pub t796: F,
    pub t797: F,
    pub t801: F,
    pub t802: F,
    pub t804: F,
    pub t805: F,
    pub t808: F,
    pub t811: F,
    pub t814: F,
    pub t817: F,
    pub t822: F,
    pub t825: F,
    pub t831: F,
    pub t832: F,
    pub t833: F,
    pub t837: F,
    pub t838: F,
    pub t839: F,
    pub t840: F,
    pub t841: F,
    pub t842: F,
    pub t843: F,
    pub t846: F,
    pub t848: F,
    pub t849: F,
    pub t853: F,
    pub t856: F,
    pub t861: F,
    pub t864: F,
    pub t870: F,
    pub t871: F,
    pub t872: F,
    pub t873: F,
    pub t874: F,
    pub t875: F,
    pub t879: F,
    pub t881: F,
    pub t887: F,
    pub t893: F,
    pub t894: F,
    pub t897: F,
    pub t900: F,
    pub t901: F,
    pub t903: F,
    pub t906: F,
    pub t907: F,
    pub t912: F,
    pub t913: F,
    pub t914: F,
    pub t917: F,
    pub t919: F,
    pub t921: F,
    pub t925: F,
    pub t928: F,
    pub t929: F,
    pub t933: F,
    pub t934: F,
    pub t935: F,
    pub t941: F,
    pub t944: F,
    pub t949: F,
    pub t954: F,
    pub t955: F,
    pub t957: F,
    pub t958: F,
    pub t959: F,
    pub t960: F,
    pub t962: F,
    pub t964: F,
    pub t965: F,
    pub t967: F,
    pub t968: F,
    pub t971: F,
    pub t974: F,
    pub t983: F,
    pub t984: F,
    pub t985: F,
    pub t986: F,
    pub t987: F,
    pub t988: F,
    pub t989: F,
    pub t990: F,
    pub t991: F,
    pub t992: F,
    pub t993: F,
    pub t994: F,
    pub t995: F,
    pub t996: F,
    pub t1000: F,
    pub t1005: F,
    pub t1008: F,
    pub t1014: F,
    pub t1015: F,
    pub t1016: F,
    pub t1017: F,
    pub t1018: F,
    pub t1019: F,
    pub t1025: F,
    pub t1031: F,
    pub tzk0: F,
    pub tvrho0: F,
    pub tvrho1: F,
    pub tvsigma0: F,
    pub tvsigma1: F,
    pub tvsigma2: F,
    pub tv2rho20: F,
    pub tv2rho21: F,
}

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_lxc_pol_chunk0<F: Float>(param_BB: F, param_beta: F, param_gamma: F, rho0: F, rho1: F, sigma0: F, sigma1: F, sigma2: F, zeta_threshold: F) -> Chunk0Out<F> {
    let cbrt3 = F::cast_from(M_CBRT3);
    let pi = F::cast_from(M_PI);
    let cbrt4 = F::cast_from(M_CBRT4);
    let cbrt2 = F::cast_from(M_CBRT2);
    let t1 = cbrt3;
    let t2 = F::cast_from(1.0_f64) / pi;
    let t3 = pow_1_3::<F>(t2);
    let t4 = t1 * t3;
    let t5 = cbrt4;
    let t6 = t5 * t5;
    let t7 = rho0 + rho1;
    let t8 = pow_1_3::<F>(t7);
    let t11 = t4 * t6 / t8;
    let t13 = F::cast_from(1.0_f64) + F::cast_from(0.53425e-1_f64) * t11;
    let t14 = F::sqrt(t11);
    let t17 = pow_3_2::<F>(t11);
    let t19 = t1 * t1;
    let t20 = t3 * t3;
    let t21 = t19 * t20;
    let t22 = t8 * t8;
    let t25 = t21 * t5 / t22;
    let t27 = F::cast_from(0.379785e1_f64) * t14 + F::cast_from(0.8969e0_f64) * t11 + F::cast_from(0.204775e0_f64) * t17 + F::cast_from(0.123235e0_f64) * t25;
    let t30 = F::cast_from(1.0_f64) + F::cast_from(0.16081979498692535067e2_f64) / t27;
    let t31 = F::ln(t30);
    let t33 = F::cast_from(0.621814e-1_f64) * t13 * t31;
    let t34 = rho0 - rho1;
    let t35 = t34 * t34;
    let t36 = t35 * t35;
    let t37 = t7 * t7;
    let t38 = t37 * t37;
    let t39 = F::cast_from(1.0_f64) / t38;
    let t40 = t36 * t39;
    let t41 = F::cast_from(1.0_f64) / t7;
    let t42 = t34 * t41;
    let t43 = F::cast_from(1.0_f64) + t42;
    let t44 = t43 <= zeta_threshold;
    let t45 = pow_1_3::<F>(zeta_threshold);
    let t46 = t45 * zeta_threshold;
    let t47 = pow_1_3::<F>(t43);
    let t48 = t47 * t43;
    let t49 = piecewise3::<F>(t44, t46, t48);
    let t50 = F::cast_from(1.0_f64) - t42;
    let t51 = t50 <= zeta_threshold;
    let t52 = pow_1_3::<F>(t50);
    let t53 = t52 * t50;
    let t54 = piecewise3::<F>(t51, t46, t53);
    let t55 = t49 + t54 - F::cast_from(2.0_f64);
    let t56 = cbrt2;
    let t59 = F::cast_from(1.0_f64) / (F::cast_from(2.0_f64) * t56 - F::cast_from(2.0_f64));
    let t60 = t55 * t59;
    let t62 = F::cast_from(1.0_f64) + F::cast_from(0.5137e-1_f64) * t11;
    let t67 = F::cast_from(0.705945e1_f64) * t14 + F::cast_from(0.1549425e1_f64) * t11 + F::cast_from(0.420775e0_f64) * t17 + F::cast_from(0.1562925e0_f64) * t25;
    let t70 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t67;
    let t71 = F::ln(t70);
    let t75 = F::cast_from(1.0_f64) + F::cast_from(0.278125e-1_f64) * t11;
    let t80 = F::cast_from(0.51785e1_f64) * t14 + F::cast_from(0.905775e0_f64) * t11 + F::cast_from(0.1100325e0_f64) * t17 + F::cast_from(0.1241775e0_f64) * t25;
    let t83 = F::cast_from(1.0_f64) + F::cast_from(0.29608749977793437516e2_f64) / t80;
    let t84 = F::ln(t83);
    let t85 = t75 * t84;
    let t87 = -F::cast_from(0.310907e-1_f64) * t62 * t71 + t33 - F::cast_from(0.19751673498613801407e-1_f64) * t85;
    let t88 = t60 * t87;
    let t89 = t40 * t88;
    let t91 = F::cast_from(0.19751673498613801407e-1_f64) * t60 * t85;
    let t92 = t45 * t45;
    let t93 = t47 * t47;
    let t94 = piecewise3::<F>(t44, t92, t93);
    let t95 = t52 * t52;
    let t96 = piecewise3::<F>(t51, t92, t95);
    let t98 = t94 / F::cast_from(2.0_f64) + t96 / F::cast_from(2.0_f64);
    let t99 = t98 * t98;
    let t100 = t99 * t98;
    let t101 = param_gamma * t100;
    let t103 = sigma0 + F::cast_from(2.0_f64) * sigma1 + sigma2;
    let t105 = F::cast_from(1.0_f64) / t8 / t37;
    let t106 = t103 * t105;
    let t108 = F::cast_from(1.0_f64) / t99;
    let t110 = F::cast_from(1.0_f64) / t3;
    let t111 = t110 * t5;
    let t112 = t108 * t19 * t111;
    let t115 = param_BB * param_beta;
    let t116 = F::cast_from(1.0_f64) / param_gamma;
    let t118 = (-t33 + t89 + t91) * t116;
    let t119 = F::cast_from(1.0_f64) / t100;
    let t121 = F::exp(-t118 * t119);
    let t122 = t121 - F::cast_from(1.0_f64);
    let t123 = F::cast_from(1.0_f64) / t122;
    let t124 = t116 * t123;
    let t125 = t103 * t103;
    let t127 = t115 * t124 * t125;
    let t129 = F::cast_from(1.0_f64) / t22 / t38;
    let t130 = t56 * t56;
    let t131 = t129 * t130;
    let t132 = t99 * t99;
    let t133 = F::cast_from(1.0_f64) / t132;
    let t134 = t131 * t133;
    let t135 = F::cast_from(1.0_f64) / t20;
    let t136 = t1 * t135;
    let t137 = t136 * t6;
    let t138 = t134 * t137;
    let t141 = t106 * t56 * t112 / F::cast_from(96.0_f64) + t127 * t138 / F::cast_from(3072.0_f64);
    let t142 = param_beta * t141;
    let t143 = param_beta * t116;
    let t146 = t143 * t123 * t141 + F::cast_from(1.0_f64);
    let t147 = F::cast_from(1.0_f64) / t146;
    let t148 = t116 * t147;
    let t150 = t142 * t148 + F::cast_from(1.0_f64);
    let t151 = F::ln(t150);
    let t152 = t101 * t151;
    let tzk0 = -t33 + t89 + t91 + t152;
    let t154 = F::cast_from(1.0_f64) / t8 / t7;
    let t155 = t6 * t154;
    let t157 = t4 * t155 * t31;
    let t158 = F::cast_from(0.11073470983333333333e-2_f64) * t157;
    let t159 = t27 * t27;
    let t160 = F::cast_from(1.0_f64) / t159;
    let t161 = t13 * t160;
    let t163 = F::cast_from(1.0_f64) / t14 * t1;
    let t164 = t3 * t6;
    let t165 = t164 * t154;
    let t166 = t163 * t165;
    let t168 = t4 * t155;
    let t170 = F::sqrt(t11);
    let t171 = t170 * t1;
    let t172 = t171 * t165;
    let t177 = t21 * t5 / t22 / t7;
    let t179 = -F::cast_from(0.632975e0_f64) * t166 - F::cast_from(0.29896666666666666667e0_f64) * t168 - F::cast_from(0.1023875e0_f64) * t172 - F::cast_from(0.82156666666666666667e-1_f64) * t177;
    let t180 = F::cast_from(1.0_f64) / t30;
    let t181 = t179 * t180;
    let t182 = t161 * t181;
    let t183 = F::cast_from(1.0_f64) * t182;
    let t184 = t35 * t34;
    let t185 = t184 * t39;
    let t186 = t185 * t88;
    let t187 = F::cast_from(4.0_f64) * t186;
    let t188 = t38 * t7;
    let t189 = F::cast_from(1.0_f64) / t188;
    let t190 = t36 * t189;
    let t191 = t190 * t88;
    let t192 = F::cast_from(4.0_f64) * t191;
    let t193 = F::cast_from(1.0_f64) / t37;
    let t194 = t34 * t193;
    let t195 = t41 - t194;
    let t198 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t195);
    let t199 = -t195;
    let t202 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t199);
    let t204 = (t198 + t202) * t59;
    let t205 = t204 * t87;
    let t206 = t40 * t205;
    let t210 = t67 * t67;
    let t211 = F::cast_from(1.0_f64) / t210;
    let t212 = t62 * t211;
    let t217 = -F::cast_from(0.1176575e1_f64) * t166 - F::cast_from(0.516475e0_f64) * t168 - F::cast_from(0.2103875e0_f64) * t172 - F::cast_from(0.104195e0_f64) * t177;
    let t218 = F::cast_from(1.0_f64) / t70;
    let t219 = t217 * t218;
    let t225 = t80 * t80;
    let t226 = F::cast_from(1.0_f64) / t225;
    let t227 = t75 * t226;
    let t232 = -F::cast_from(0.86308333333333333334e0_f64) * t166 - F::cast_from(0.301925e0_f64) * t168 - F::cast_from(0.5501625e-1_f64) * t172 - F::cast_from(0.82785e-1_f64) * t177;
    let t233 = F::cast_from(1.0_f64) / t83;
    let t234 = t232 * t233;
    let t237 = F::cast_from(0.53237641966666666666e-3_f64) * t4 * t155 * t71 + F::cast_from(1.0_f64) * t212 * t219 - t158 - t183 + F::cast_from(0.18311447306006545054e-3_f64) * t4 * t155 * t84 + F::cast_from(0.5848223622634646207e0_f64) * t227 * t234;
    let t238 = t60 * t237;
    let t239 = t40 * t238;
    let t240 = t204 * t85;
    let t241 = F::cast_from(0.19751673498613801407e-1_f64) * t240;
    let t242 = t60 * t1;
    let t244 = t164 * t154 * t84;
    let t245 = t242 * t244;
    let t246 = F::cast_from(0.18311447306006545054e-3_f64) * t245;
    let t247 = t60 * t75;
    let t249 = t226 * t232 * t233;
    let t250 = t247 * t249;
    let t251 = F::cast_from(0.5848223622634646207e0_f64) * t250;
    let t252 = param_gamma * t99;
    let t253 = F::cast_from(1.0_f64) / t47;
    let t256 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t253 * t195);
    let t257 = F::cast_from(1.0_f64) / t52;
    let t260 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t257 * t199);
    let t262 = t256 / F::cast_from(2.0_f64) + t260 / F::cast_from(2.0_f64);
    let t263 = t151 * t262;
    let t264 = t252 * t263;
    let t265 = F::cast_from(3.0_f64) * t264;
    let t266 = t37 * t7;
    let t268 = F::cast_from(1.0_f64) / t8 / t266;
    let t269 = t103 * t268;
    let t272 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t269 * t56 * t112;
    let t273 = t56 * t119;
    let t274 = t106 * t273;
    let t275 = t19 * t110;
    let t276 = t5 * t262;
    let t277 = t275 * t276;
    let t280 = t115 * t116;
    let t281 = t122 * t122;
    let t282 = F::cast_from(1.0_f64) / t281;
    let t283 = t282 * t125;
    let t285 = t280 * t283 * t129;
    let t286 = t130 * t133;
    let t287 = t286 * t1;
    let t288 = t135 * t6;
    let t290 = (t158 + t183 + t187 - t192 + t206 + t239 + t241 - t246 - t251) * t116;
    let t292 = t133 * t262;
    let t295 = F::cast_from(3.0_f64) * t118 * t292 - t290 * t119;
    let t296 = t295 * t121;
    let t297 = t288 * t296;
    let t298 = t287 * t297;
    let t302 = F::cast_from(1.0_f64) / t22 / t188;
    let t303 = t302 * t130;
    let t304 = t303 * t133;
    let t305 = t304 * t137;
    let t307 = F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t127 * t305;
    let t308 = t123 * t125;
    let t310 = t280 * t308 * t129;
    let t312 = F::cast_from(1.0_f64) / t132 / t98;
    let t313 = t130 * t312;
    let t314 = t313 * t1;
    let t316 = t314 * t288 * t262;
    let t319 = -t272 - t274 * t277 / F::cast_from(48.0_f64) - t285 * t298 / F::cast_from(3072.0_f64) - t307 - t310 * t316 / F::cast_from(768.0_f64);
    let t320 = param_beta * t319;
    let t322 = t146 * t146;
    let t323 = F::cast_from(1.0_f64) / t322;
    let t324 = t116 * t323;
    let t325 = t143 * t282;
    let t326 = t141 * t295;
    let t331 = -t325 * t326 * t121 + t143 * t123 * t319;
    let t332 = t324 * t331;
    let t334 = -t142 * t332 + t320 * t148;
    let t335 = F::cast_from(1.0_f64) / t150;
    let t336 = t334 * t335;
    let t337 = t101 * t336;
    let t338 = t158 + t183 + t187 - t192 + t206 + t239 + t241 - t246 - t251 + t265 + t337;
    let tvrho0 = t7 * t338 + t152 - t33 + t89 + t91;
    let t340 = -t41 - t194;
    let t343 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t340);
    let t344 = -t340;
    let t347 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t344);
    let t349 = (t343 + t347) * t59;
    let t350 = t349 * t87;
    let t351 = t40 * t350;
    let t352 = t349 * t85;
    let t353 = F::cast_from(0.19751673498613801407e-1_f64) * t352;
    let t356 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t253 * t340);
    let t359 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t257 * t344);
    let t361 = t356 / F::cast_from(2.0_f64) + t359 / F::cast_from(2.0_f64);
    let t362 = t151 * t361;
    let t363 = t252 * t362;
    let t364 = F::cast_from(3.0_f64) * t363;
    let t365 = t5 * t361;
    let t366 = t275 * t365;
    let t370 = (t158 + t183 - t187 - t192 + t351 + t239 + t353 - t246 - t251) * t116;
    let t372 = t133 * t361;
    let t375 = F::cast_from(3.0_f64) * t118 * t372 - t370 * t119;
    let t376 = t375 * t121;
    let t377 = t288 * t376;
    let t378 = t287 * t377;
    let t382 = t314 * t288 * t361;
    let t385 = -t272 - t274 * t366 / F::cast_from(48.0_f64) - t285 * t378 / F::cast_from(3072.0_f64) - t307 - t310 * t382 / F::cast_from(768.0_f64);
    let t386 = param_beta * t385;
    let t388 = t141 * t375;
    let t393 = -t325 * t388 * t121 + t143 * t123 * t385;
    let t394 = t324 * t393;
    let t396 = -t142 * t394 + t386 * t148;
    let t397 = t396 * t335;
    let t398 = t101 * t397;
    let t399 = t158 + t183 - t187 - t192 + t351 + t239 + t353 - t246 - t251 + t364 + t398;
    let tvrho1 = t7 * t399 + t152 - t33 + t89 + t91;
    let t401 = t7 * param_gamma;
    let t402 = t105 * t56;
    let t404 = t275 * t5;
    let t405 = t402 * t108 * t404;
    let t408 = t115 * t124 * t103;
    let t409 = t408 * t138;
    let t411 = t405 / F::cast_from(96.0_f64) + t409 / F::cast_from(1536.0_f64);
    let t412 = param_beta * t411;
    let t414 = param_beta * param_beta;
    let t415 = t414 * t141;
    let t416 = param_gamma * param_gamma;
    let t417 = F::cast_from(1.0_f64) / t416;
    let t418 = t415 * t417;
    let t419 = t323 * t123;
    let t420 = t419 * t411;
    let t422 = t412 * t148 - t418 * t420;
    let tvsigma0 = t401 * t100 * t422 * t335;
    let t427 = t405 / F::cast_from(48.0_f64) + t409 / F::cast_from(768.0_f64);
    let t428 = param_beta * t427;
    let t430 = t419 * t427;
    let t432 = t428 * t148 - t418 * t430;
    let t433 = t100 * t432;
    let tvsigma1 = t401 * t433 * t335;
    let tvsigma2 = tvsigma0;
    let t435 = F::cast_from(0.22146941966666666666e-2_f64) * t157;
    let t436 = F::cast_from(2.0_f64) * t182;
    let t437 = F::cast_from(8.0_f64) * t186;
    let t438 = F::cast_from(8.0_f64) * t191;
    let t440 = F::cast_from(2.0_f64) * t239;
    let t442 = F::cast_from(0.36622894612013090108e-3_f64) * t245;
    let t443 = F::cast_from(0.11696447245269292414e1_f64) * t250;
    let t448 = F::cast_from(1.0_f64) / t14 / t11 * t19;
    let t449 = t20 * t5;
    let t451 = F::cast_from(1.0_f64) / t22 / t37;
    let t452 = t449 * t451;
    let t453 = t448 * t452;
    let t455 = t164 * t105;
    let t456 = t163 * t455;
    let t458 = t6 * t105;
    let t459 = t4 * t458;
    let t461 = F::cast_from(1.0_f64)/F::sqrt(t11);
    let t462 = t461 * t19;
    let t463 = t462 * t452;
    let t465 = t171 * t455;
    let t468 = t21 * t5 * t451;
    let t470 = -F::cast_from(0.42198333333333333333e0_f64) * t453 + F::cast_from(0.84396666666666666666e0_f64) * t456 + F::cast_from(0.39862222222222222223e0_f64) * t459 + F::cast_from(0.68258333333333333333e-1_f64) * t463 + F::cast_from(0.13651666666666666667e0_f64) * t465 + F::cast_from(0.13692777777777777778e0_f64) * t468;
    let t471 = t470 * t180;
    let t472 = t161 * t471;
    let t473 = F::cast_from(1.0_f64) * t472;
    let t474 = t159 * t159;
    let t475 = F::cast_from(1.0_f64) / t474;
    let t476 = t13 * t475;
    let t477 = t179 * t179;
    let t478 = t30 * t30;
    let t479 = F::cast_from(1.0_f64) / t478;
    let t480 = t477 * t479;
    let t481 = t476 * t480;
    let t482 = F::cast_from(0.16081979498692535067e2_f64) * t481;
    let t483 = t159 * t27;
    let t484 = F::cast_from(1.0_f64) / t483;
    let t485 = t13 * t484;
    let t486 = t477 * t180;
    let t487 = t485 * t486;
    let t488 = F::cast_from(2.0_f64) * t487;
    let t489 = t225 * t225;
    let t490 = F::cast_from(1.0_f64) / t489;
    let t491 = t232 * t232;
    let t492 = t490 * t491;
    let t493 = t83 * t83;
    let t494 = F::cast_from(1.0_f64) / t493;
    let t495 = t492 * t494;
    let t496 = t247 * t495;
    let t497 = F::cast_from(0.17315859105681463759e2_f64) * t496;
    let t504 = -F::cast_from(0.57538888888888888889e0_f64) * t453 + F::cast_from(0.11507777777777777778e1_f64) * t456 + F::cast_from(0.40256666666666666667e0_f64) * t459 + F::cast_from(0.366775e-1_f64) * t463 + F::cast_from(0.73355e-1_f64) * t465 + F::cast_from(0.137975e0_f64) * t468;
    let t506 = t226 * t504 * t233;
    let t507 = t247 * t506;
    let t508 = F::cast_from(0.5848223622634646207e0_f64) * t507;
    let t509 = t204 * t75;
    let t510 = t509 * t249;
    let t511 = F::cast_from(0.11696447245269292414e1_f64) * t510;
    let t512 = F::cast_from(1.0_f64) / t93;
    let t513 = t195 * t195;
    let t516 = F::cast_from(1.0_f64) / t266;
    let t517 = t34 * t516;
    let t519 = -F::cast_from(2.0_f64) * t193 + F::cast_from(2.0_f64) * t517;
    let t523 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t512 * t513 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t519);
    let t524 = F::cast_from(1.0_f64) / t95;
    let t525 = t199 * t199;
    let t528 = -t519;
    let t532 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t524 * t525 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t528);
    let t534 = (t523 + t532) * t59;
    let t535 = t534 * t85;
    let t536 = F::cast_from(0.19751673498613801407e-1_f64) * t535;
    let t537 = t4 * t6;
    let t538 = t154 * t160;
    let t540 = t537 * t538 * t181;
    let t541 = F::cast_from(0.35616666666666666666e-1_f64) * t540;
    let t542 = t204 * t1;
    let t543 = t542 * t244;
    let t544 = F::cast_from(0.36622894612013090108e-3_f64) * t543;
    let t546 = t164 * t105 * t84;
    let t547 = t242 * t546;
    let t548 = F::cast_from(0.24415263074675393405e-3_f64) * t547;
    let t549 = t60 * t4;
    let t550 = t155 * t249;
    let t551 = t549 * t550;
    let t552 = F::cast_from(0.10843581300301739842e-1_f64) * t551;
    let t553 = t225 * t80;
    let t554 = F::cast_from(1.0_f64) / t553;
    let t556 = t554 * t491 * t233;
    let t557 = t247 * t556;
    let t558 = F::cast_from(0.11696447245269292414e1_f64) * t557;
    let t559 = param_gamma * t98;
    let t560 = t262 * t262;
    let t562 = t559 * t151 * t560;
    let t563 = F::cast_from(6.0_f64) * t562;
    let t564 = F::cast_from(1.0_f64) / t48;
    let t570 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t564 * t513 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t253 * t519);
    let t571 = F::cast_from(1.0_f64) / t53;
    let t577 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t571 * t525 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t257 * t528);
    let t579 = t570 / F::cast_from(2.0_f64) + t577 / F::cast_from(2.0_f64);
    let t580 = t151 * t579;
    let t581 = t252 * t580;
    let t582 = F::cast_from(3.0_f64) * t581;
    let t583 = t473 + t482 - t488 - t497 - t508 - t511 + t536 - t541 - t544 + t548 + t552 + t558 + t563 + t582;
    let t585 = F::cast_from(1.0_f64) / t8 / t38;
    let t586 = t103 * t585;
    let t589 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t586 * t56 * t112;
    let t590 = t269 * t273;
    let t591 = t590 * t277;
    let t593 = t56 * t133;
    let t594 = t106 * t593;
    let t595 = t5 * t560;
    let t596 = t275 * t595;
    let t600 = t275 * t5 * t579;
    let t604 = F::cast_from(1.0_f64) / t281 / t122;
    let t605 = t604 * t125;
    let t607 = t280 * t605 * t129;
    let t608 = t295 * t295;
    let t609 = t121 * t121;
    let t610 = t608 * t609;
    let t612 = t287 * t288 * t610;
    let t616 = t280 * t283 * t302;
    let t617 = t616 * t298;
    let t620 = t280 * t283 * t131;
    let t621 = t312 * t1;
    let t622 = t621 * t135;
    let t623 = t6 * t295;
    let t624 = t121 * t262;
    let t625 = t623 * t624;
    let t626 = t622 * t625;
    let t632 = t154 * t211;
    let t636 = t210 * t67;
    let t637 = F::cast_from(1.0_f64) / t636;
    let t638 = t62 * t637;
    let t639 = t217 * t217;
    let t640 = t639 * t218;
    let t649 = -F::cast_from(0.78438333333333333333e0_f64) * t453 + F::cast_from(0.15687666666666666667e1_f64) * t456 + F::cast_from(0.68863333333333333333e0_f64) * t459 + F::cast_from(0.14025833333333333333e0_f64) * t463 + F::cast_from(0.28051666666666666667e0_f64) * t465 + F::cast_from(0.17365833333333333333e0_f64) * t468;
    let t650 = t649 * t218;
    let t653 = t210 * t210;
    let t654 = F::cast_from(1.0_f64) / t653;
    let t655 = t62 * t654;
    let t656 = t70 * t70;
    let t657 = F::cast_from(1.0_f64) / t656;
    let t658 = t639 * t657;
    let t662 = t4 * t458 * t31;
    let t663 = F::cast_from(0.14764627977777777777e-2_f64) * t662;
    let t667 = t154 * t226;
    let t671 = t75 * t554;
    let t672 = t491 * t233;
    let t675 = t504 * t233;
    let t678 = t75 * t490;
    let t679 = t491 * t494;
    let t682 = -F::cast_from(0.70983522622222222221e-3_f64) * t4 * t458 * t71 - F::cast_from(0.34246666666666666666e-1_f64) * t537 * t632 * t219 - F::cast_from(2.0_f64) * t638 * t640 + F::cast_from(1.0_f64) * t212 * t650 + F::cast_from(0.32163958997385070134e2_f64) * t655 * t658 + t663 + t541 + t488 - t473 - t482 - F::cast_from(0.24415263074675393405e-3_f64) * t4 * t458 * t84 - F::cast_from(0.10843581300301739842e-1_f64) * t537 * t667 * t234 - F::cast_from(0.11696447245269292414e1_f64) * t671 * t672 + F::cast_from(0.5848223622634646207e0_f64) * t227 * t675 + F::cast_from(0.17315859105681463759e2_f64) * t678 * t679;
    let t683 = t60 * t682;
    let t684 = t40 * t683;
    let t685 = t534 * t87;
    let t686 = t40 * t685;
    let t687 = t204 * t237;
    let t688 = t40 * t687;
    let t689 = F::cast_from(2.0_f64) * t688;
    let t690 = t190 * t205;
    let t691 = F::cast_from(8.0_f64) * t690;
    let t692 = t190 * t238;
    let t693 = F::cast_from(8.0_f64) * t692;
    let t694 = t185 * t205;
    let t695 = F::cast_from(8.0_f64) * t694;
    let t696 = t185 * t238;
    let t697 = F::cast_from(8.0_f64) * t696;
    let t698 = t536 - t497 - t508 - t511 + t684 + t686 + t689 - t691 - t693 + t695 + t697;
    let t699 = t38 * t37;
    let t700 = F::cast_from(1.0_f64) / t699;
    let t701 = t36 * t700;
    let t702 = t701 * t88;
    let t703 = F::cast_from(20.0_f64) * t702;
    let t704 = t35 * t39;
    let t705 = t704 * t88;
    let t706 = F::cast_from(12.0_f64) * t705;
    let t707 = t184 * t189;
    let t708 = t707 * t88;
    let t709 = F::cast_from(32.0_f64) * t708;
    let t710 = -t488 + t703 + t706 - t709 - t663 - t544 + t558 - t541 + t552 + t473 + t482 + t548;
    let t712 = (t698 + t710) * t116;
    let t716 = t312 * t560;
    let t719 = t133 * t579;
    let t722 = -F::cast_from(12.0_f64) * t118 * t716 + F::cast_from(3.0_f64) * t118 * t719 - t712 * t119 + F::cast_from(6.0_f64) * t290 * t292;
    let t723 = t722 * t121;
    let t725 = t287 * t288 * t723;
    let t728 = t608 * t121;
    let t730 = t287 * t288 * t728;
    let t734 = F::cast_from(1.0_f64) / t22 / t699;
    let t735 = t734 * t130;
    let t737 = t735 * t133 * t137;
    let t739 = F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t127 * t737;
    let t741 = t280 * t308 * t302;
    let t742 = t741 * t316;
    let t745 = F::cast_from(1.0_f64) / t132 / t99;
    let t746 = t130 * t745;
    let t747 = t746 * t1;
    let t749 = t747 * t288 * t560;
    let t753 = t314 * t288 * t579;
    let t756 = t589 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t591 + t594 * t596 / F::cast_from(16.0_f64) - t274 * t600 / F::cast_from(48.0_f64) + t607 * t612 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t617 + t620 * t626 / F::cast_from(384.0_f64) - t285 * t725 / F::cast_from(3072.0_f64) - t285 * t730 / F::cast_from(3072.0_f64) + t739 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t742 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t310 * t749 - t310 * t753 / F::cast_from(768.0_f64);
    let t757 = param_beta * t756;
    let t762 = F::cast_from(1.0_f64) / t322 / t146;
    let t763 = t116 * t762;
    let t764 = t331 * t331;
    let t765 = t763 * t764;
    let t768 = t143 * t604;
    let t769 = t141 * t608;
    let t773 = t319 * t295;
    let t777 = t141 * t722;
    let t784 = -t325 * t769 * t121 - F::cast_from(2.0_f64) * t325 * t773 * t121 - t325 * t777 * t121 + t143 * t123 * t756 + F::cast_from(2.0_f64) * t768 * t769 * t609;
    let t785 = t324 * t784;
    let t787 = F::cast_from(2.0_f64) * t142 * t765 - t142 * t785 + t757 * t148 - F::cast_from(2.0_f64) * t320 * t332;
    let t788 = t787 * t335;
    let t789 = t101 * t788;
    let t790 = t334 * t334;
    let t791 = t150 * t150;
    let t792 = F::cast_from(1.0_f64) / t791;
    let t793 = t790 * t792;
    let t794 = t101 * t793;
    let t796 = t252 * t336 * t262;
    let t797 = F::cast_from(6.0_f64) * t796;
    let t798 = t789 + t697 + t703 + t706 - t709 - t693 + t695 + t689 - t691 + t686 - t663 + t684 - t794 + t797;
    let tv2rho20 = t435 + t436 + t437 - t438 + F::cast_from(2.0_f64) * t206 + t440 + F::cast_from(0.39503346997227602814e-1_f64) * t240 - t442 - t443 + F::cast_from(6.0_f64) * t264 + F::cast_from(2.0_f64) * t337 + t7 * (t583 + t798);
    let t801 = t349 * t75;
    let t802 = t801 * t249;
    let t803 = F::cast_from(0.5848223622634646207e0_f64) * t802;
    let t804 = t349 * t1;
    let t805 = t804 * t244;
    let t806 = F::cast_from(0.18311447306006545054e-3_f64) * t805;
    let t808 = t590 * t366;
    let t811 = t275 * t365 * t262;
    let t814 = t564 * t340;
    let t817 = t253 * t34;
    let t821 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t814 * t195 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t817 * t516);
    let t822 = t571 * t344;
    let t825 = t257 * t34;
    let t829 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t822 * t199 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t825 * t516);
    let t831 = t821 / F::cast_from(2.0_f64) + t829 / F::cast_from(2.0_f64);
    let t832 = t5 * t831;
    let t833 = t275 * t832;
    let t837 = t280 * t605 * t131;
    let t838 = t133 * t1;
    let t839 = t838 * t135;
    let t840 = t6 * t375;
    let t841 = t609 * t295;
    let t842 = t840 * t841;
    let t843 = t839 * t842;
    let t846 = t616 * t378;
    let t848 = t840 * t624;
    let t849 = t622 * t848;
    let t852 = F::cast_from(0.5848223622634646207e0_f64) * t510;
    let t853 = t512 * t340;
    let t856 = t47 * t34;
    let t860 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t853 * t195 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t856 * t516);
    let t861 = t524 * t344;
    let t864 = t52 * t34;
    let t868 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t861 * t199 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t864 * t516);
    let t870 = (t860 + t868) * t59;
    let t871 = t870 * t87;
    let t872 = t40 * t871;
    let t873 = t349 * t237;
    let t874 = t40 * t873;
    let t875 = t870 * t85;
    let t876 = F::cast_from(0.19751673498613801407e-1_f64) * t875;
    let t877 = F::cast_from(0.18311447306006545054e-3_f64) * t543;
    let t878 = -t803 + t473 + t482 - t488 - t806 - t497 - t508 - t852 + t872 + t874 + t876 - t541 - t877;
    let t879 = t185 * t350;
    let t880 = F::cast_from(4.0_f64) * t879;
    let t881 = t190 * t350;
    let t882 = F::cast_from(4.0_f64) * t881;
    let t883 = F::cast_from(4.0_f64) * t694;
    let t884 = F::cast_from(4.0_f64) * t690;
    let t885 = t548 + t552 + t558 + t880 - t882 + t703 - t706 - t693 - t883 + t688 - t884 - t663 + t684;
    let t887 = (t878 + t885) * t116;
    let t893 = t312 * t361;
    let t894 = t893 * t262;
    let t897 = t133 * t831;
    let t900 = -F::cast_from(12.0_f64) * t118 * t894 + F::cast_from(3.0_f64) * t118 * t897 - t887 * t119 + F::cast_from(3.0_f64) * t290 * t372 + F::cast_from(3.0_f64) * t370 * t292;
    let t901 = t900 * t121;
    let t903 = t287 * t288 * t901;
    let t906 = t840 * t296;
    let t907 = t839 * t906;
    let t912 = t6 * t361;
    let t913 = t912 * t296;
    let t914 = t622 * t913;
    let t917 = t741 * t382;
    let t919 = t361 * t262;
    let t921 = t747 * t288 * t919;
    let t925 = t314 * t288 * t831;
    let t928 = t589 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t591 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t808 + t594 * t811 / F::cast_from(16.0_f64) - t274 * t833 / F::cast_from(48.0_f64) + t837 * t843 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t846 + t620 * t849 / F::cast_from(768.0_f64) - t285 * t903 / F::cast_from(3072.0_f64) - t620 * t907 / F::cast_from(3072.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t617 + t739 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t742 + t620 * t914 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t917 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t310 * t921 - t310 * t925 / F::cast_from(768.0_f64);
    let t929 = param_beta * t928;
    let t933 = t142 * t116;
    let t934 = t762 * t393;
    let t935 = t934 * t331;
    let t941 = t319 * t375;
    let t944 = t141 * t900;
    let t949 = t385 * t295;
    let t954 = -t325 * t941 * t121 - t325 * t944 * t121 - t325 * t949 * t121 + t143 * t123 * t928 - t325 * t388 * t296 + F::cast_from(2.0_f64) * t768 * t388 * t841;
    let t955 = t324 * t954;
    let t957 = -t142 * t955 + t929 * t148 - t320 * t394 - t386 * t332 + F::cast_from(2.0_f64) * t933 * t935;
    let t958 = t957 * t335;
    let t959 = t101 * t958;
    let t960 = t396 * t792;
    let t962 = t101 * t960 * t334;
    let t963 = -t803 + t473 + t482 - t488 - t806 + t959 - t962 - t497 - t508 - t852 + t872 + t874 + t876 - t541 - t877 + t548;
    let t964 = t336 * t361;
    let t965 = t252 * t964;
    let t967 = t151 * t831;
    let t968 = t252 * t967;
    let t971 = t252 * t397 * t262;
    let t974 = t559 * t362 * t262;
    let t976 = t552 + t558 + t880 - t882 + t703 - t706 - t693 - t883 + t688 - t884 + F::cast_from(3.0_f64) * t965 - t663 + t684 + F::cast_from(3.0_f64) * t968 + F::cast_from(3.0_f64) * t971 + F::cast_from(6.0_f64) * t974;
    let tv2rho21 = t435 + t436 - t438 + t206 + t440 + t241 - t442 - t443 + t265 + t337 + t351 + t353 + t364 + t398 + t7 * (t963 + t976);
    let t983 = F::cast_from(0.11696447245269292414e1_f64) * t802;
    let t984 = F::cast_from(0.36622894612013090108e-3_f64) * t805;
    let t985 = t397 * t361;
    let t986 = t252 * t985;
    let t987 = F::cast_from(6.0_f64) * t986;
    let t988 = F::cast_from(2.0_f64) * t874;
    let t989 = F::cast_from(8.0_f64) * t879;
    let t990 = -t983 + t473 + t482 - t488 - t984 + t987 - t497 - t508 + t988 - t541 + t548 + t552 + t558 - t989;
    let t991 = F::cast_from(8.0_f64) * t881;
    let t992 = t361 * t361;
    let t993 = t151 * t992;
    let t994 = t559 * t993;
    let t995 = F::cast_from(6.0_f64) * t994;
    let t996 = t340 * t340;
    let t1000 = F::cast_from(2.0_f64) * t193 + F::cast_from(2.0_f64) * t517;
    let t1004 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t512 * t996 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t1000);
    let t1005 = t344 * t344;
    let t1008 = -t1000;
    let t1012 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t524 * t1005 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t1008);
    let t1014 = (t1004 + t1012) * t59;
    let t1015 = t1014 * t85;
    let t1016 = F::cast_from(0.19751673498613801407e-1_f64) * t1015;
    let t1017 = t396 * t396;
    let t1018 = t1017 * t792;
    let t1019 = t101 * t1018;
    let t1025 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t564 * t996 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t253 * t1000);
    let t1031 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t571 * t1005 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t257 * t1008);
    Chunk0Out::<F> { t1: t1, t2: t2, t3: t3, t4: t4, t5: t5, t6: t6, t7: t7, t8: t8, t11: t11, t13: t13, t14: t14, t19: t19, t21: t21, t22: t22, t25: t25, t27: t27, t30: t30, t31: t31, t34: t34, t35: t35, t36: t36, t38: t38, t39: t39, t40: t40, t41: t41, t43: t43, t47: t47, t50: t50, t52: t52, t55: t55, t56: t56, t59: t59, t60: t60, t62: t62, t67: t67, t70: t70, t71: t71, t75: t75, t80: t80, t83: t83, t84: t84, t85: t85, t87: t87, t88: t88, t93: t93, t95: t95, t98: t98, t99: t99, t100: t100, t101: t101, t103: t103, t105: t105, t106: t106, t108: t108, t111: t111, t112: t112, t115: t115, t116: t116, t118: t118, t119: t119, t121: t121, t122: t122, t123: t123, t125: t125, t127: t127, t129: t129, t130: t130, t131: t131, t132: t132, t133: t133, t134: t134, t135: t135, t136: t136, t137: t137, t138: t138, t141: t141, t142: t142, t143: t143, t146: t146, t147: t147, t148: t148, t150: t150, t151: t151, t154: t154, t155: t155, t159: t159, t160: t160, t161: t161, t163: t163, t164: t164, t168: t168, t171: t171, t179: t179, t180: t180, t181: t181, t184: t184, t185: t185, t188: t188, t189: t189, t190: t190, t195: t195, t199: t199, t204: t204, t205: t205, t210: t210, t211: t211, t212: t212, t217: t217, t218: t218, t219: t219, t225: t225, t226: t226, t227: t227, t232: t232, t233: t233, t234: t234, t237: t237, t238: t238, t242: t242, t244: t244, t247: t247, t249: t249, t252: t252, t253: t253, t257: t257, t262: t262, t263: t263, t266: t266, t268: t268, t269: t269, t273: t273, t274: t274, t275: t275, t276: t276, t277: t277, t280: t280, t281: t281, t282: t282, t283: t283, t285: t285, t286: t286, t287: t287, t288: t288, t290: t290, t292: t292, t295: t295, t296: t296, t297: t297, t298: t298, t302: t302, t303: t303, t304: t304, t305: t305, t308: t308, t310: t310, t312: t312, t313: t313, t314: t314, t316: t316, t319: t319, t320: t320, t322: t322, t323: t323, t324: t324, t325: t325, t326: t326, t331: t331, t332: t332, t334: t334, t335: t335, t336: t336, t340: t340, t344: t344, t349: t349, t350: t350, t351: t351, t352: t352, t361: t361, t362: t362, t363: t363, t365: t365, t366: t366, t370: t370, t372: t372, t375: t375, t376: t376, t377: t377, t378: t378, t382: t382, t385: t385, t386: t386, t388: t388, t393: t393, t394: t394, t396: t396, t397: t397, t398: t398, t401: t401, t402: t402, t404: t404, t408: t408, t411: t411, t412: t412, t414: t414, t415: t415, t416: t416, t417: t417, t418: t418, t419: t419, t420: t420, t422: t422, t427: t427, t428: t428, t430: t430, t432: t432, t433: t433, t435: t435, t436: t436, t437: t437, t438: t438, t440: t440, t442: t442, t443: t443, t448: t448, t449: t449, t458: t458, t462: t462, t470: t470, t471: t471, t472: t472, t473: t473, t474: t474, t475: t475, t476: t476, t477: t477, t478: t478, t479: t479, t480: t480, t481: t481, t482: t482, t483: t483, t484: t484, t485: t485, t486: t486, t487: t487, t488: t488, t489: t489, t490: t490, t491: t491, t492: t492, t493: t493, t494: t494, t495: t495, t496: t496, t497: t497, t504: t504, t506: t506, t507: t507, t508: t508, t509: t509, t510: t510, t511: t511, t512: t512, t513: t513, t516: t516, t517: t517, t519: t519, t524: t524, t525: t525, t528: t528, t534: t534, t535: t535, t536: t536, t537: t537, t538: t538, t540: t540, t541: t541, t542: t542, t543: t543, t544: t544, t546: t546, t547: t547, t548: t548, t549: t549, t550: t550, t551: t551, t552: t552, t553: t553, t554: t554, t556: t556, t557: t557, t558: t558, t559: t559, t560: t560, t562: t562, t563: t563, t564: t564, t571: t571, t579: t579, t580: t580, t581: t581, t582: t582, t585: t585, t586: t586, t589: t589, t590: t590, t593: t593, t594: t594, t595: t595, t596: t596, t600: t600, t604: t604, t605: t605, t607: t607, t608: t608, t609: t609, t610: t610, t612: t612, t616: t616, t620: t620, t621: t621, t622: t622, t623: t623, t624: t624, t625: t625, t626: t626, t632: t632, t636: t636, t637: t637, t638: t638, t639: t639, t640: t640, t649: t649, t650: t650, t653: t653, t654: t654, t655: t655, t656: t656, t657: t657, t658: t658, t662: t662, t663: t663, t667: t667, t671: t671, t672: t672, t675: t675, t678: t678, t679: t679, t682: t682, t683: t683, t684: t684, t685: t685, t686: t686, t687: t687, t688: t688, t689: t689, t690: t690, t691: t691, t692: t692, t693: t693, t694: t694, t695: t695, t696: t696, t697: t697, t699: t699, t700: t700, t701: t701, t702: t702, t703: t703, t704: t704, t705: t705, t706: t706, t707: t707, t708: t708, t709: t709, t712: t712, t716: t716, t719: t719, t722: t722, t723: t723, t725: t725, t728: t728, t730: t730, t734: t734, t735: t735, t737: t737, t739: t739, t741: t741, t745: t745, t746: t746, t747: t747, t749: t749, t753: t753, t756: t756, t757: t757, t762: t762, t763: t763, t764: t764, t765: t765, t768: t768, t769: t769, t773: t773, t777: t777, t784: t784, t785: t785, t787: t787, t788: t788, t789: t789, t790: t790, t791: t791, t792: t792, t793: t793, t794: t794, t796: t796, t797: t797, t801: t801, t802: t802, t804: t804, t805: t805, t808: t808, t811: t811, t814: t814, t817: t817, t822: t822, t825: t825, t831: t831, t832: t832, t833: t833, t837: t837, t838: t838, t839: t839, t840: t840, t841: t841, t842: t842, t843: t843, t846: t846, t848: t848, t849: t849, t853: t853, t856: t856, t861: t861, t864: t864, t870: t870, t871: t871, t872: t872, t873: t873, t874: t874, t875: t875, t879: t879, t881: t881, t887: t887, t893: t893, t894: t894, t897: t897, t900: t900, t901: t901, t903: t903, t906: t906, t907: t907, t912: t912, t913: t913, t914: t914, t917: t917, t919: t919, t921: t921, t925: t925, t928: t928, t929: t929, t933: t933, t934: t934, t935: t935, t941: t941, t944: t944, t949: t949, t954: t954, t955: t955, t957: t957, t958: t958, t959: t959, t960: t960, t962: t962, t964: t964, t965: t965, t967: t967, t968: t968, t971: t971, t974: t974, t983: t983, t984: t984, t985: t985, t986: t986, t987: t987, t988: t988, t989: t989, t990: t990, t991: t991, t992: t992, t993: t993, t994: t994, t995: t995, t996: t996, t1000: t1000, t1005: t1005, t1008: t1008, t1014: t1014, t1015: t1015, t1016: t1016, t1017: t1017, t1018: t1018, t1019: t1019, t1025: t1025, t1031: t1031, tzk0: tzk0, tvrho0: tvrho0, tvrho1: tvrho1, tvsigma0: tvsigma0, tvsigma1: tvsigma1, tvsigma2: tvsigma2, tv2rho20: tv2rho20, tv2rho21: tv2rho21 }
}
